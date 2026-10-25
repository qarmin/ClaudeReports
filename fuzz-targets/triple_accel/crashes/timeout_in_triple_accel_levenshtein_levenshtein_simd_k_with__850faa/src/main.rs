use triple_accel::levenshtein::EditCosts;

fn main() {
    let _ = triple_accel::levenshtein::levenshtein_simd_k_with_opts(&vec![], &include_bytes!("data_0.bin").to_vec(), 2428045950u32, false, EditCosts::new(55u8, 52u8, 134u8, None));
}
