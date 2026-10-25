use blake2::Blake2bVarCore;

fn main() {
    let _ = Blake2bVarCore::new_with_params(&vec![], &vec![73u8, 73u8, 42u8, 0u8, 42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8], 0usize, 0usize);
}
