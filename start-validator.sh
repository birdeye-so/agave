#!/bin/bash

# PATH=$HOME/.local/share/solana/install/active_release/bin:/usr/sbin:/usr/bin:/sbin:/bin:/home/ubuntu/agave/target/release
export RUST_BACKTRACE=1
export RUST_LOG=solana=info

export MALLOC_CONF="prof:true,lg_prof_interval:32,lg_prof_sample:17,prof_prefix:/tmp/agave/jeprof"

cargo build --profile release-with-debug --bin solana-validator --features jemallocator/profiling

exec agave-validator \
  --identity ~/.solana_keys/validator-keypair.json \
  --entrypoint entrypoint.mainnet-beta.solana.com:8001 \
  --entrypoint entrypoint2.mainnet-beta.solana.com:8001 \
  --entrypoint entrypoint3.mainnet-beta.solana.com:8001 \
  --entrypoint entrypoint4.mainnet-beta.solana.com:8001 \
  --entrypoint entrypoint5.mainnet-beta.solana.com:8001 \
  --dynamic-port-range 8000-8100 \
  --gossip-port 8001 \
  --no-voting \
  --private-rpc \
  --rpc-bind-address 0.0.0.0 \
  --rpc-port 8899 \
  --rpc-threads 16 --full-rpc-api \
  --wal-recovery-mode skip_any_corrupted_record \
  --log ~/log/agave-validator.log \
  --limit-ledger-size 50000000 \
  --ledger /ledger/ledger \
  --accounts /accounts \
  --snapshots /ledger/snapshots \
  --health-check-slot-distance 150 \
  --known-validator 7Np41oeYqPefeNQEHSv1UDhYrehxin3NStELsSKCT4K2 \
  --known-validator GdnSyH3YtwcxFvQrVVJMm1JhTS4QVX7MFsX56uJLUfiZ \
  --known-validator DE1bawNcRJB9rVm3buyMVfr8mBEoyyu73NBovf2oXJsJ \
  --known-validator CakcnaRDHka2gXyfbEd2d3xsvkJkqsLw2akB3zsN1D2S \
  --minimal-snapshot-download-speed 1048576000 \
  --accounts-index-limit 25GB --accounts-db-write-cache-limit 15GB --vote-account ~/.solana_keys/vote-account-keypair.json \
  --expected-genesis-hash 5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d \
  --geyser-plugin-config /home/ubuntu/yellowstone-grpc/yellowstone-grpc-geyser/config.json \
  --no-snapshots
