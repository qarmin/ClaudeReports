fn main() {
    let vec = triple_accel::alloc_str(65536usize);
    let _ = triple_accel::levenshtein::levenshtein_simd_k(&vec, &vec![], 1000000000u32);
}
