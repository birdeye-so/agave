fn partition_key(pubkey: &Pubkey, num_partitions: usize) -> String {
    // TODO: calculate partition key based on pubkey
    // with hash algorithm or first, last and middle byte of pubkey
    // and mod to number of partitions
    pubkey.to_string()
}
