use brotli::enc::static_dict::FindMatchLengthWithLimitMin4;

fn main() {
    let _ = FindMatchLengthWithLimitMin4(&vec![0u8, 0u8, 0u8, 0u8, 0u8], &vec![], 0usize);
}
