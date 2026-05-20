use std::{error::Error, net::SocketAddr, result::Result, str::FromStr, sync::Arc};

use log::{debug, warn};
use solana_account::{AccountSharedData, ReadableAccount};
use solana_accounts_db::{
    accounts_index::{AccountIndex, IndexKey, ScanConfig, ScanError},
    is_loadable::IsLoadable,
};
use solana_address::Address;
use solana_runtime::bank::Bank;
use tokio_stream::wrappers::ReceiverStream;

use crate::pb::account::v1::{
    Account, FILE_DESCRIPTOR_SET, ListProgramAccountsRequest, RpcContextConfig,
    account_service_server::{AccountService, AccountServiceServer},
};

pub trait BankManager: Send + Sync + 'static {
    fn get_bank_with_config(&self, config: RpcContextConfig) -> Result<Arc<Bank>, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct AccountServer {
    bank_manager: Arc<dyn BankManager>,
}

impl AccountServer {
    pub fn new(bank_manager: Arc<dyn BankManager>) -> Self {
        Self { bank_manager }
    }
}

#[tonic::async_trait]
impl AccountService for AccountServer {
    #[doc = " Server streaming response type for the ListProgramAccounts method."]
    type ListProgramAccountsStream = ReceiverStream<Result<Account, tonic::Status>>;

    async fn list_program_accounts(
        &self,
        req: tonic::Request<ListProgramAccountsRequest>,
    ) -> Result<tonic::Response<Self::ListProgramAccountsStream>, tonic::Status> {
        let program_id = solana_address::Address::from_str(&req.get_ref().program_id)
            .map_err(|_| tonic::Status::invalid_argument("invalid program_id"))?;

        if req.get_ref().discriminators.iter().any(|d| d.len() != 8) {
            return Err(tonic::Status::invalid_argument("invalid discriminator"));
        }

        let discriminators = req.get_ref().discriminators.clone();

        let bank = self
            .bank_manager
            .get_bank_with_config(req.get_ref().config.unwrap_or_default())
            .map_err(|e| tonic::Status::unknown(e.to_string()))?;

        let (tx, rx) = tokio::sync::mpsc::channel(1024);

        tokio::task::spawn_blocking(move || {
            let scan_config = ScanConfig::default();

            let scan_func = |account: Option<(&Address, AccountSharedData, u64)>| {
                if let Some((pubkey, account, slot)) = account {
                    if !account.is_loadable() {
                        return;
                    }

                    if &program_id != account.owner() {
                        return;
                    }

                    if !discriminators.is_empty()
                        && discriminators
                            .iter()
                            .all(|d| !account.data().starts_with(d))
                    {
                        return;
                    }

                    let msg = Ok(Account {
                        pubkey: pubkey.as_array().to_vec(),
                        slot,
                        lamports: account.lamports(),
                        data: account.data().to_vec(),
                        owner: account.owner().as_array().to_vec(),
                        executable: account.executable(),
                        rent_epoch: account.rent_epoch(),
                    });

                    if let Err(e) = tx.blocking_send(msg) {
                        warn!("failed to send data: {e}");
                        scan_config.abort();
                    }
                }
            };

            let scan_result = match bank
                .accounts()
                .accounts_db
                .account_indexes
                .contains(&AccountIndex::ProgramId)
            {
                true => bank.accounts().accounts_db.index_scan_accounts(
                    &bank.ancestors,
                    bank.bank_id(),
                    IndexKey::ProgramId(program_id),
                    scan_func,
                    &scan_config,
                ),
                false => bank
                    .accounts()
                    .accounts_db
                    .scan_accounts(&bank.ancestors, bank.bank_id(), scan_func, &scan_config)
                    .map(|_| false),
            };

            if let Err(e) = scan_result {
                match e {
                    ScanError::SlotRemoved { slot, bank_id: _ } => {
                        let _ = tx.blocking_send(Err(tonic::Status::data_loss(format!(
                            "slot {slot} removed"
                        ))));
                    }
                    ScanError::Aborted(_) => {
                        debug!("stream aborted");
                    }
                }
            }
        });

        Ok(tonic::Response::new(ReceiverStream::new(rx)))
    }
}

pub async fn start_server(
    addr: SocketAddr,
    bank_manager: Arc<dyn BankManager>,
) -> Result<(), tonic::transport::Error> {
    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let account_server = AccountServer::new(bank_manager);

    tonic::transport::Server::builder()
        .add_service(reflection)
        .add_service(AccountServiceServer::new(account_server))
        .serve(addr)
        .await
}
