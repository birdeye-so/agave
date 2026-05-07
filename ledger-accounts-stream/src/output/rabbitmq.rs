use {solana_cli_output::CliAccount, solana_pubkey::Pubkey};

use super::{AccountOutput, TotalAccountsStats};

pub struct RabbitMqOutput {
    pub url: String,
    pub exchange: String,
    pub routing_key: String,
}

impl RabbitMqOutput {
    pub fn new(
        url: impl Into<String>,
        exchange: impl Into<String>,
        routing_key: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            exchange: exchange.into(),
            routing_key: routing_key.into(),
        }
    }
}

impl AccountOutput for RabbitMqOutput {
    fn begin(&mut self) -> Result<(), String> {
        // TODO: open AMQP connection to self.url and declare self.exchange
        Ok(())
    }

    fn write_account(
        &mut self,
        pubkey: &Pubkey,
        cli_account: &CliAccount,
        slot: u64,
    ) -> Result<(), String> {
        // TODO: serialize cli_account and slot as byte buffer and publish to self.exchange
        //       using pubkey.to_string() as the routing key
        let _ = (pubkey, cli_account, slot);
        Ok(())
    }

    fn end(&mut self, stats: &TotalAccountsStats) -> Result<(), String> {
        // TODO: publish stats as a final summary message and close the connection
        let _ = stats;
        Ok(())
    }
}
