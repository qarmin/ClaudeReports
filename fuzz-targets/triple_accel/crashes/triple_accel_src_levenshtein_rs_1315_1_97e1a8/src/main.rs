use triple_accel::levenshtein::RDAMERAU_COSTS;

fn main() {
    let edit_costs = RDAMERAU_COSTS;
    let _ = triple_accel::levenshtein::levenshtein_simd_k_with_opts(&include_bytes!("data_0.bin").to_vec(), &vec![255u8], 1000000000u32, true, edit_costs);
}
