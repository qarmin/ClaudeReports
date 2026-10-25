use triple_accel::levenshtein::RDAMERAU_COSTS;

fn main() {
    let _ = triple_accel::levenshtein::levenshtein_simd_k_with_opts(&vec![128u8], &include_bytes!("data_0.bin").to_vec(), 1000000000u32, true, RDAMERAU_COSTS);
}
