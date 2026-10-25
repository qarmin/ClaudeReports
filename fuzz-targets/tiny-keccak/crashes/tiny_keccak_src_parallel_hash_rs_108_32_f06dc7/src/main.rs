use tiny_keccak::ParallelHash;

fn main() {
    let mut parallel_hash = ParallelHash::v256(&vec![], 0usize);
    <tiny_keccak::ParallelHash as tiny_keccak::Hasher>::update(&mut parallel_hash, &vec![]);
}
