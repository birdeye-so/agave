use clap::{ArgMatches, value_t_or_exit};
use log::*;
use solana_clap_utils::input_parsers::{pubkey_of, pubkeys_of};
use solana_cli_output::OutputFormat;
use solana_measure::measure_time;
use std::{path::Path, sync::Arc};

use crate::{
    args::{parse_account_output_config, parse_process_options},
    ledger_path::canonicalize_ledger_path,
    ledger_utils::{
        LoadAndProcessLedgerOutput, get_access_type, load_and_process_ledger_or_exit,
        open_blockstore, open_genesis_config_by,
    },
    output::std_out::StdOutOutput,
    output::{
        AccountOutput, AccountsOutputConfig, AccountsOutputMode, AccountsOutputStreamer,
        JetStreamOutput,
    },
};

pub fn accounts(path: &Path, arg_matches: &ArgMatches<'_>) {
    let ledger_path = canonicalize_ledger_path(path);
    let process_options = parse_process_options(&ledger_path, arg_matches);
    let genesis_config = open_genesis_config_by(&ledger_path, arg_matches);
    let blockstore = open_blockstore(&ledger_path, arg_matches, get_access_type(&process_options));
    let LoadAndProcessLedgerOutput { bank_forks, .. } = load_and_process_ledger_or_exit(
        arg_matches,
        &genesis_config,
        Arc::new(blockstore),
        process_options,
        None,
    );
    let bank = bank_forks.read().unwrap().working_bank();

    let include_sysvars = arg_matches.is_present("include_sysvars");
    let output_config = if arg_matches.is_present("no_account_contents") {
        None
    } else {
        Some(parse_account_output_config(arg_matches))
    };

    let mode = if let Some(pubkeys) = pubkeys_of(arg_matches, "account") {
        info!("Scanning individual accounts: {pubkeys:?}");
        AccountsOutputMode::Individual(pubkeys)
    } else if let Some(pubkey) = pubkey_of(arg_matches, "program_accounts") {
        info!("Scanning program accounts for {pubkey}");
        AccountsOutputMode::Program(pubkey)
    } else {
        info!("Scanning all accounts");
        AccountsOutputMode::All
    };
    let config = AccountsOutputConfig {
        mode,
        output_config,
        include_sysvars,
    };

    let output: Box<dyn AccountOutput> = if let Some(url) = arg_matches.value_of("nats_url") {
        let stream = arg_matches.value_of("nats_stream").unwrap_or("solana");
        let subject = arg_matches
            .value_of("nats_subject")
            .unwrap_or("solana.accounts");
        let token = arg_matches.value_of("nats_token").map(|s| s.to_string());
        let partitions = value_t_or_exit!(arg_matches, "nats_partitions", usize);
        Box::new(JetStreamOutput::new(
            url, stream, subject, token, partitions,
        ))
    } else {
        let output_format = OutputFormat::from_matches(arg_matches, "output_format", false);
        Box::new(StdOutOutput::new(output_format))
    };

    let accounts_streamer = AccountsOutputStreamer::new(bank, config, output);
    let (_, scan_time) = measure_time!(
        accounts_streamer
            .output()
            .map_err(|err| error!("Error while outputting accounts: {err}")),
        "accounts scan"
    );
    info!("{scan_time}");
}
