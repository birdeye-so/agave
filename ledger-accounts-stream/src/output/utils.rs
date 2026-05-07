use solana_pubkey::Pubkey;

pub fn get_partition(pubkey: &Pubkey, num_partitions: usize) -> usize {
    if num_partitions <= 1 {
        return 1;
    }
    let bytes = pubkey.as_ref();
    let first = bytes[0] as u64;
    let middle = bytes[bytes.len() / 2] as u64;
    let last = bytes[bytes.len() - 1] as u64;

    let sum = first + middle + last;
    (sum % num_partitions as u64) as usize + 1
}
