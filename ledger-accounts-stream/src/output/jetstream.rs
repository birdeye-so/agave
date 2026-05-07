use {
    async_nats::jetstream,
    crossbeam_channel::{Sender, unbounded},
    serde::{Deserialize, Serialize},
    solana_cli_output::CliAccount,
    solana_pubkey::Pubkey,
    std::thread::{self, JoinHandle},
    tokio::runtime::Runtime,
};

use super::{AccountOutput, TotalAccountsStats, utils::get_partition};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawAccount {
    pub pubkey: String,
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
    pub data: Vec<u8>,
    pub slot: u64,
}

pub struct JetStreamOutput {
    pub url: String,
    pub stream: String,
    pub subject: String,
    pub token: Option<String>,
    pub num_partitions: usize,
    sender: Option<Sender<Option<(Pubkey, RawAccount)>>>,
    thread: Option<JoinHandle<()>>,
}

impl JetStreamOutput {
    pub fn new(
        url: impl Into<String>,
        stream: impl Into<String>,
        subject: impl Into<String>,
        token: Option<String>,
        num_partitions: usize,
    ) -> Self {
        Self {
            url: url.into(),
            stream: stream.into(),
            subject: subject.into(),
            token,
            num_partitions,
            sender: None,
            thread: None,
        }
    }
}

impl AccountOutput for JetStreamOutput {
    fn begin(&mut self) -> Result<(), String> {
        let (tx, rx) = unbounded::<Option<(Pubkey, RawAccount)>>();
        let url = self.url.clone();
        let stream_name = self.stream.clone();
        let subject_prefix = self.subject.clone();
        let num_partitions = self.num_partitions;
        let token = self.token.clone();

        let handle = thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let mut connect_options = async_nats::ConnectOptions::new();
                if let Some(token) = token {
                    connect_options = connect_options.token(token);
                }
                let client = match connect_options.connect(&url).await {
                    Ok(client) => client,
                    Err(e) => {
                        eprintln!("Failed to connect to NATS at {}: {}", url, e);
                        return;
                    }
                };
                log::info!(
                    "Connected to NATS at {}, stream: {}, subject: {}",
                    url,
                    stream_name,
                    subject_prefix
                );
                let js = jetstream::new(client);

                // Ensure the stream exists
                let stream_subject = format!("{}.>", subject_prefix);
                match js.get_stream(&stream_name).await {
                    Ok(_) => {
                        log::info!("Stream '{}' already exists", stream_name);
                    }
                    Err(_) => {
                        log::info!(
                            "Creating stream '{}' with subject '{}'",
                            stream_name,
                            stream_subject
                        );
                        if let Err(e) = js
                            .create_stream(async_nats::jetstream::stream::Config {
                                name: stream_name.clone(),
                                subjects: vec![stream_subject],
                                ..Default::default()
                            })
                            .await
                        {
                            eprintln!("Failed to create JetStream stream '{}': {}", stream_name, e);
                            return;
                        }
                    }
                }

                while let Ok(Some((pubkey, account))) = rx.recv() {
                    let partition = get_partition(&pubkey, num_partitions);
                    let data = bincode::serialize(&account).unwrap();

                    // Push to partitioned subject: solana.accounts.partition.1
                    let partition_subject = format!("{}.partition.{}", subject_prefix, partition);
                    if let Err(e) = js.publish(partition_subject, data.clone().into()).await {
                        eprintln!("Failed to publish to JetStream: {}", e);
                    }

                    // Push to all subject: solana.accounts.all
                    let all_subject = format!("{}.all", subject_prefix);
                    if let Err(e) = js.publish(all_subject, data.into()).await {
                        eprintln!("Failed to publish to JetStream (all): {}", e);
                    }
                }
            });
        });

        self.sender = Some(tx);
        self.thread = Some(handle);
        Ok(())
    }

    fn write_account(
        &mut self,
        pubkey: &Pubkey,
        cli_account: &CliAccount,
        slot: u64,
    ) -> Result<(), String> {
        if let Some(sender) = &self.sender {
            let data = cli_account
                .keyed_account
                .account
                .data
                .decode()
                .unwrap_or_default();
            let raw_account = RawAccount {
                pubkey: pubkey.to_string(),
                lamports: cli_account.keyed_account.account.lamports,
                owner: cli_account.keyed_account.account.owner.clone(),
                executable: cli_account.keyed_account.account.executable,
                rent_epoch: cli_account.keyed_account.account.rent_epoch,
                data,
                slot,
            };
            sender
                .send(Some((*pubkey, raw_account)))
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn end(&mut self, _stats: &TotalAccountsStats) -> Result<(), String> {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(None);
        }
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
        Ok(())
    }
}
