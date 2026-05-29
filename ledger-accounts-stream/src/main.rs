#![allow(clippy::arithmetic_side_effects)]
use {
    crate::{accounts::*, args::*, ledger_path::*, program::*},
    clap::{
        App, AppSettings, Arg, SubCommand, crate_description, crate_name, value_t, value_t_or_exit,
    },
    log::*,
    solana_clap_utils::input_validators::{is_parsable, is_pubkey, is_within_range},
    solana_core::{resource_limits::adjust_nofile_limit, validator::BlockVerificationMethod},
    solana_measure::measure::Measure,
    solana_unified_scheduler_pool::DefaultSchedulerPool,
    std::{path::PathBuf, process::exit},
};

mod accounts;
mod args;
mod ledger_path;
mod ledger_utils;
mod output;
mod program;

#[cfg(not(any(target_env = "msvc", target_os = "freebsd")))]
use jemallocator::Jemalloc;

#[cfg(not(any(target_env = "msvc", target_os = "freebsd")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[allow(clippy::cognitive_complexity)]
fn main() {
    // Ignore SIGUSR1 to prevent long-running calls being killed by logrotate
    // in warehouse deployments
    #[cfg(unix)]
    {
        // `register()` is unsafe because the action is called in a signal handler
        // with the usual caveats. So long as this action body stays empty, we'll
        // be fine
        unsafe { signal_hook::low_level::register(signal_hook::consts::SIGUSR1, || {}) }.unwrap();
    }

    let load_genesis_config_arg = load_genesis_arg();
    let accounts_db_config_args = accounts_db_args();
    let snapshot_config_args = snapshot_args();

    let geyser_plugin_args = Arg::with_name("geyser_plugin_config")
        .long("geyser-plugin-config")
        .value_name("FILE")
        .takes_value(true)
        .multiple(true)
        .help("Specify the configuration file for the Geyser plugin.");

    let log_messages_bytes_limit_arg = Arg::with_name("log_messages_bytes_limit")
        .long("log-messages-bytes-limit")
        .takes_value(true)
        .validator(is_parsable::<usize>)
        .value_name("BYTES")
        .help("Maximum number of bytes written to the program log before truncation");

    let accounts_data_encoding_arg = Arg::with_name("encoding")
        .long("encoding")
        .takes_value(true)
        .possible_values(&["base64", "base64+zstd", "jsonParsed"])
        .default_value("base64")
        .help("Print account data in specified format when printing account contents.");

    let mut measure_total_execution_time = Measure::start("ledger tool");

    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(solana_version::version!())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::InferSubcommands)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("ledger_path")
                .short("l")
                .long("ledger")
                .value_name("DIR")
                .takes_value(true)
                .global(true)
                .default_value("ledger")
                .help("Use DIR as ledger location"),
        )
        .arg(
            Arg::with_name("logfile")
                .long("log")
                .value_name("FILE")
                .takes_value(true)
                .global(true)
                .help("Redirect logging to the specified file, stderr is used if unset"),
        )
        .arg(
            Arg::with_name("wal_recovery_mode")
                .long("wal-recovery-mode")
                .value_name("MODE")
                .takes_value(true)
                .global(true)
                .possible_values(&[
                    "tolerate_corrupted_tail_records",
                    "absolute_consistency",
                    "point_in_time",
                    "skip_any_corrupted_record",
                ])
                .help("Mode to recovery the ledger db write ahead log"),
        )
        .arg(
            Arg::with_name("force_update_to_open")
                .long("force-update-to-open")
                .takes_value(false)
                .global(true)
                .help(
                    "Allow commands that would otherwise not alter the blockstore to make \
                     necessary updates in order to open it",
                ),
        )
        .arg(
            Arg::with_name("ignore_ulimit_nofile_error")
                .long("ignore-ulimit-nofile-error")
                .takes_value(false)
                .global(true)
                .help(
                    "Allow the command to continue even if the desired open file descriptor limit \
                     cannot be configured. Use with caution as some commands may run fine with a \
                     a reduced file descriptor limit while others may fail in nonobvious ways",
                ),
        )
        .arg(
            Arg::with_name("block_verification_method")
                .long("block-verification-method")
                .value_name("METHOD")
                .takes_value(true)
                .possible_values(BlockVerificationMethod::cli_names())
                .default_value(BlockVerificationMethod::default().into())
                .global(true)
                .help(BlockVerificationMethod::cli_message()),
        )
        .arg(
            Arg::with_name("unified_scheduler_handler_threads")
                .long("unified-scheduler-handler-threads")
                .value_name("COUNT")
                .takes_value(true)
                .validator(|s| is_within_range(s, 1..))
                .global(true)
                .help(DefaultSchedulerPool::cli_message()),
        )
        .arg(
            Arg::with_name("output_format")
                .long("output")
                .value_name("FORMAT")
                .global(true)
                .takes_value(true)
                .possible_values(&["json", "json-compact"])
                .help(
                    "Return information in specified output format, currently only available for \
                     bigtable and program subcommands",
                ),
        )
        // All of the blockstore commands are added under the blockstore command.
        // For the sake of legacy support, also directly add the blockstore commands here so that
        // these subcommands can continue to be called from the top level of the binary.
        .subcommand(
            SubCommand::with_name("accounts")
                .about("Print account stats and contents after processing the ledger")
                .arg(&load_genesis_config_arg)
                .args(&accounts_db_config_args)
                .args(&snapshot_config_args)
                .arg(&geyser_plugin_args)
                .arg(&log_messages_bytes_limit_arg)
                .arg(&accounts_data_encoding_arg)
                .arg(
                    Arg::with_name("include_sysvars")
                        .long("include-sysvars")
                        .takes_value(false)
                        .help("Include sysvars too"),
                )
                .arg(
                    Arg::with_name("no_account_contents")
                        .long("no-account-contents")
                        .takes_value(false)
                        .help(
                            "Do not print contents of each account, which is very slow with lots \
                             of accounts.",
                        ),
                )
                .arg(
                    Arg::with_name("no_account_data")
                        .long("no-account-data")
                        .takes_value(false)
                        .help("Do not print account data when printing account contents."),
                )
                .arg(
                    Arg::with_name("account")
                        .long("account")
                        .takes_value(true)
                        .value_name("PUBKEY")
                        .validator(is_pubkey)
                        .multiple(true)
                        .help(
                            "Limit output to accounts corresponding to the specified pubkey(s), \
                             may be specified multiple times",
                        ),
                )
                .arg(
                    Arg::with_name("program_accounts")
                        .long("program-accounts")
                        .takes_value(true)
                        .value_name("PUBKEY")
                        .validator(is_pubkey)
                        .conflicts_with("account")
                        .help("Limit output to accounts owned by the provided program pubkey"),
                )
                .arg(
                    Arg::with_name("nats_url")
                        .long("nats-url")
                        .takes_value(true)
                        .value_name("URL")
                        .help("NATS server URL"),
                )
                .arg(
                    Arg::with_name("nats_stream")
                        .long("nats-stream")
                        .takes_value(true)
                        .value_name("STREAM")
                        .help("NATS JetStream stream name"),
                )
                .arg(
                    Arg::with_name("nats_subject")
                        .long("nats-subject")
                        .takes_value(true)
                        .value_name("SUBJECT")
                        .default_value("solana.accounts")
                        .help("NATS JetStream subject prefix"),
                )
                .arg(
                    Arg::with_name("nats_partitions")
                        .long("nats-partitions")
                        .takes_value(true)
                        .value_name("NUMBER")
                        .validator(is_parsable::<usize>)
                        .default_value("10")
                        .help("Number of partitions for JetStream"),
                )
                .arg(
                    Arg::with_name("nats_token")
                        .long("nats-token")
                        .takes_value(true)
                        .value_name("TOKEN")
                        .help("NATS authentication token"),
                ),
        )
        .program_subcommand()
        .get_matches();

    let logfile = value_t!(matches, "logfile", PathBuf).ok();
    agave_logger::initialize_logging(logfile);

    info!("{} {}", crate_name!(), solana_version::version!());

    let ledger_path = PathBuf::from(value_t_or_exit!(matches, "ledger_path", String));

    let enforce_nofile_limit = !matches.is_present("ignore_ulimit_nofile_error");
    adjust_nofile_limit(enforce_nofile_limit).unwrap_or_else(|err| {
        eprintln!("Error: {err:?}");
        exit(1);
    });

    // Name the rayon global thread pool
    rayon::ThreadPoolBuilder::new()
        .thread_name(|i| format!("solRayonGlob{i:02}"))
        .build_global()
        .unwrap();

    match matches.subcommand() {
        ("program", Some(arg_matches)) => program(&ledger_path, arg_matches),
        ("accounts", Some(arg_matches)) => accounts(&ledger_path, arg_matches),
        _ => unreachable!(),
    };
    measure_total_execution_time.stop();
    info!("{measure_total_execution_time}");
}
