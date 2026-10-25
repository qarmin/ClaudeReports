use brotli::enc::command::BrotliDistanceParams;
use brotli::enc::command::Command;
use brotli::enc::command::RecomputeDistancePrefixes;

fn main() {
    RecomputeDistancePrefixes(&mut vec![Command { insert_len_: 0u32, copy_len_: 1u32, dist_extra_: 0u32, cmd_prefix_: 1000u16, dist_prefix_: u16::MAX }], 1usize, 0u32, 1u32, &BrotliDistanceParams { distance_postfix_bits: 0u32, num_direct_distance_codes: 0u32, alphabet_size: 0u32, max_distance: 0usize });
}
