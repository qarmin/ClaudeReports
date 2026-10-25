fn main() {
    let hashes = phf_shared::hash::<u16>(&0u16, &0u64);
    let _ = phf_shared::get_index(&hashes, &[], 1usize);
}
