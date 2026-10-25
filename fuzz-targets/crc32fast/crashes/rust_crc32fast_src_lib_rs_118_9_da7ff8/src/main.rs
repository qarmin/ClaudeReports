use crc32fast::Hasher;

fn main() {
    let u32_v = crc32fast::hash(&vec![]);
    let mut hasher = Hasher::new_with_initial_len(u32_v, u64::MAX);
    hasher.update(&vec![34u8]);
}
