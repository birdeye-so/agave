use {
    serde::Serialize,
    solana_account::{AccountSharedData, ReadableAccount as _},
    solana_accounts_db::is_loadable::IsLoadable as _,
    solana_cli_output::{CliAccount, CliAccountNewConfig},
    solana_pubkey::Pubkey,
    solana_runtime::bank::Bank,
    std::{cell::RefCell, sync::Arc},
};

pub mod jetstream;
pub mod rabbitmq;
pub mod std_out;

/// Implemented by each output destination (stdout, RabbitMQ, JetStream, …).
/// The streamer calls `begin` once, then `write_account` for every matching
/// account, then `end` with the final stats.
pub trait AccountOutput {
    fn begin(&mut self) -> Result<(), String>;
    fn write_account(&mut self, pubkey: &Pubkey, cli_account: &CliAccount) -> Result<(), String>;
    fn end(&mut self, stats: &TotalAccountsStats) -> Result<(), String>;
}

pub enum AccountsOutputMode {
    All,
    Individual(Vec<Pubkey>),
    Program(Pubkey),
}

pub struct AccountsOutputConfig {
    pub mode: AccountsOutputMode,
    /// `None` when `--no-account-contents` is set; accounts are counted but
    /// not written to the output sink.
    pub output_config: Option<CliAccountNewConfig>,
    pub include_sysvars: bool,
}

#[derive(Debug, Default, Copy, Clone, Serialize)]
pub struct TotalAccountsStats {
    pub num_accounts: usize,
    pub data_len: usize,
    pub num_executable_accounts: usize,
    pub executable_data_len: usize,
}

impl TotalAccountsStats {
    pub fn accumulate_account(&mut self, account: &AccountSharedData) {
        let data_len = account.data().len();
        self.num_accounts += 1;
        self.data_len += data_len;
        if account.executable() {
            self.num_executable_accounts += 1;
            self.executable_data_len += data_len;
        }
    }
}

pub struct AccountsOutputStreamer {
    bank: Arc<Bank>,
    config: AccountsOutputConfig,
    output: RefCell<Box<dyn AccountOutput>>,
}

impl AccountsOutputStreamer {
    pub fn new(bank: Arc<Bank>, config: AccountsOutputConfig, output: Box<dyn AccountOutput>) -> Self {
        Self {
            bank,
            config,
            output: RefCell::new(output),
        }
    }

    fn is_included(&self, account: &AccountSharedData) -> bool {
        account.is_loadable()
            && (self.config.include_sysvars
                || !solana_sdk_ids::sysvar::check_id(account.owner()))
    }

    /// Accumulates stats and, when `output_config` is set, forwards the
    /// account to the output sink.
    fn emit_account(
        &self,
        pubkey: &Pubkey,
        account: &AccountSharedData,
        stats: &mut TotalAccountsStats,
    ) {
        stats.accumulate_account(account);
        if let Some(cfg) = self.config.output_config.as_ref() {
            let cli_account = CliAccount::new_with_config(pubkey, account, cfg);
            self.output
                .borrow_mut()
                .write_account(pubkey, &cli_account)
                .unwrap();
        }
    }

    pub fn output(&self) -> Result<(), String> {
        let mut stats = TotalAccountsStats::default();
        self.output.borrow_mut().begin()?;

        match &self.config.mode {
            AccountsOutputMode::All => {
                self.bank
                    .scan_all_accounts(|account_tuple| {
                        if let Some((pubkey, account, _slot)) =
                            account_tuple.filter(|(_, account, _)| self.is_included(account))
                        {
                            self.emit_account(pubkey, &account, &mut stats);
                        }
                    })
                    .map_err(|err| format!("scan error: {err}"))?;
            }

            AccountsOutputMode::Individual(pubkeys) => {
                for pubkey in pubkeys {
                    if let Some((account, _slot)) = self
                        .bank
                        .get_account_modified_slot_with_fixed_root(pubkey)
                        .filter(|(account, _)| self.is_included(account))
                    {
                        self.emit_account(pubkey, &account, &mut stats);
                    }
                }
            }

            AccountsOutputMode::Program(program_pubkey) => {
                for (pubkey, account) in self
                    .bank
                    .get_program_accounts(program_pubkey)
                    .map_err(|err| format!("get_program_accounts error: {err}"))?
                    .iter()
                    .filter(|(_, account)| self.is_included(account))
                {
                    self.emit_account(pubkey, account, &mut stats);
                }
            }
        }

        self.output.borrow_mut().end(&stats)?;
        Ok(())
    }
}
