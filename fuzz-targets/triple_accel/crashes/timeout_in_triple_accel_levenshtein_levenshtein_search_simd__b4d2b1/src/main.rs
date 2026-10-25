use triple_accel::SearchType;
use triple_accel::levenshtein::RDAMERAU_COSTS;

fn main() {
    let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(&include_bytes!("data_0.bin").to_vec(), &include_bytes!("data_1.bin").to_vec(), 255u32, SearchType::Best, RDAMERAU_COSTS, true);
}
