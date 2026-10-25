use triple_accel::SearchType;
use triple_accel::levenshtein::RDAMERAU_COSTS;

fn main() {
    let _ = triple_accel::levenshtein::levenshtein_search_naive_with_opts(&include_bytes!("data_0.bin").to_vec(), &include_bytes!("image_1.webp").to_vec(), 4294967294u32, SearchType::Best, RDAMERAU_COSTS, false);
}
