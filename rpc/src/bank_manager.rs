use std::{error::Error, sync::Arc};

use grpc::pb::account::v1::RpcContextConfig;
use solana_commitment_config::CommitmentConfig;
use solana_runtime::bank::Bank;

use crate::rpc::JsonRpcRequestProcessor;

pub struct BankManager {
    request_processor: JsonRpcRequestProcessor,
}

impl BankManager {
    pub fn new(request_processor: JsonRpcRequestProcessor) -> Self {
        return Self { request_processor };
    }
}

impl grpc::server::BankManager for BankManager {
    fn get_bank_with_config(&self, config: RpcContextConfig) -> Result<Arc<Bank>, Box<dyn Error>> {
        let config = solana_client::rpc_config::RpcContextConfig {
            commitment: match config.commitment {
                None => None,
                Some(v) => match v {
                    1 => Some(CommitmentConfig::processed()),
                    2 => Some(CommitmentConfig::confirmed()),
                    3 => Some(CommitmentConfig::finalized()),
                    _ => None,
                },
            },
            min_context_slot: config.min_context_slot,
        };

        let bank = self.request_processor.get_bank_with_config(config)?;

        Ok(bank)
    }
}
