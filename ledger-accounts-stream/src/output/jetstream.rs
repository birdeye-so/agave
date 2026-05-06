use {solana_cli_output::CliAccount, solana_pubkey::Pubkey};

use super::{AccountOutput, TotalAccountsStats};

pub struct JetStreamOutput {
    pub url: String,
    pub stream: String,
    pub subject: String,
}

impl JetStreamOutput {
    pub fn new(
        url: impl Into<String>,
        stream: impl Into<String>,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            stream: stream.into(),
            subject: subject.into(),
        }
    }
}

impl AccountOutput for JetStreamOutput {
    fn begin(&mut self) -> Result<(), String> {
        // TODO: connect to NATS at self.url and set up a JetStream context
        //       for self.stream
        Ok(())
    }

    fn write_account(&mut self, pubkey: &Pubkey, cli_account: &CliAccount) -> Result<(), String> {
        // TODO: serialize cli_account as byte buffer and publish to self.subject
        let _ = (pubkey, cli_account);
        Ok(())
    }

    fn end(&mut self, stats: &TotalAccountsStats) -> Result<(), String> {
        // TODO: publish stats as a final summary message and flush the connection
        let _ = stats;
        Ok(())
    }
}
