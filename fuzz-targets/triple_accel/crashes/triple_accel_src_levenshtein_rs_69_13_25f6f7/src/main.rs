use triple_accel::SearchType;
use triple_accel::levenshtein::EditCosts;

fn main() {
    let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(&vec![66u8], &vec![], 0u32, SearchType::Best, EditCosts::new(100u8, 63u8, 1u8, Some(100u8)), false);
}
