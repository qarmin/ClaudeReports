use triple_accel::SearchType;
use triple_accel::levenshtein::RDAMERAU_COSTS;

fn main() {
    let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(&vec![0u8, 0u8], &vec![], u32::MAX, SearchType::Best, RDAMERAU_COSTS, true);
}
