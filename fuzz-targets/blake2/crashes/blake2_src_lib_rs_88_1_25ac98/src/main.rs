use blake2::Blake2sVarCore;

fn main() {
    let _ = Blake2sVarCore::new_with_params(&vec![], &vec![], 1000000000000000000usize, 0usize);
}
