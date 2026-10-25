use brotli::enc::command::BrotliDistanceParams;
use brotli::enc::command::Command;

fn main() {
    let command = Command { insert_len_: 0u32, copy_len_: 0u32, dist_extra_: 0u32, cmd_prefix_: 0u16, dist_prefix_: 100u16 };
    let _ = command.distance_index_and_offset(&BrotliDistanceParams { distance_postfix_bits: 0u32, num_direct_distance_codes: 1u32, alphabet_size: 0u32, max_distance: 0usize });
}
