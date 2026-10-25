use brotli::enc::command::BrotliDistanceParams;
use brotli::enc::command::RecomputeDistancePrefixes;

fn main() {
    RecomputeDistancePrefixes(&mut vec![], 1usize, 0u32, 1u32, &BrotliDistanceParams { distance_postfix_bits: 1u32, num_direct_distance_codes: 0u32, alphabet_size: 0u32, max_distance: 1usize });
}
