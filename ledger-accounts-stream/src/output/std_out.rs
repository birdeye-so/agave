use {
    pretty_hex::PrettyHex,
    solana_cli_output::{CliAccount, OutputFormat},
    solana_pubkey::Pubkey,
};

use super::{AccountOutput, TotalAccountsStats};

pub struct StdOutOutput {
    format: OutputFormat,
    is_first_account: bool,
}

impl StdOutOutput {
    pub fn new(format: OutputFormat) -> Self {
        Self {
            format,
            is_first_account: true,
        }
    }
}

impl AccountOutput for StdOutOutput {
    fn begin(&mut self) -> Result<(), String> {
        if matches!(self.format, OutputFormat::Json | OutputFormat::JsonCompact) {
            print!("{{\"accounts\":[");
        }
        Ok(())
    }

    fn write_account(&mut self, _pubkey: &Pubkey, cli_account: &CliAccount) -> Result<(), String> {
        match self.format {
            OutputFormat::Json | OutputFormat::JsonCompact => {
                if !self.is_first_account {
                    print!(",");
                }
                self.is_first_account = false;
                let json = serde_json::to_string(cli_account)
                    .map_err(|e| format!("serialization error: {e}"))?;
                print!("{json}");
            }
            _ => {
                print!("{cli_account}");
                let account_data = cli_account.keyed_account.account.data.decode();
                if let Some(data) = account_data {
                    if !data.is_empty() {
                        println!("{:?}", data.hex_dump());
                    }
                }
            }
        }
        Ok(())
    }

    fn end(&mut self, stats: &TotalAccountsStats) -> Result<(), String> {
        match self.format {
            OutputFormat::Json | OutputFormat::JsonCompact => {
                let summary = serde_json::to_string(stats)
                    .map_err(|e| format!("serialization error: {e}"))?;
                println!("],\"summary\":{summary}}}");
            }
            _ => {
                println!("\n{:#?}", stats);
            }
        }
        Ok(())
    }
}
