use brotli::enc::command::BrotliDistanceParams;
use brotli::enc::command::Command;

fn main() {
    let _ = Command::new(&BrotliDistanceParams { distance_postfix_bits: 0u32, num_direct_distance_codes: 0u32, alphabet_size: 0u32, max_distance: 0usize }, 0usize, 0usize, 1usize, 0usize);
}
