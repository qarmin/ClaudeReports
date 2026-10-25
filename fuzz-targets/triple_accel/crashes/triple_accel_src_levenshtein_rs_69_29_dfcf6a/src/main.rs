use triple_accel::SearchType;
use triple_accel::levenshtein::EditCosts;

fn main() {
    let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(&vec![82u8], &vec![], 0u32, SearchType::Best, EditCosts::new(1u8, 1u8, 255u8, Some(1u8)), false);
}
