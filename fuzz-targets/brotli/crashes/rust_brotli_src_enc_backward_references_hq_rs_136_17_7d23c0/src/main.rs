use brotli::enc::BrotliEncoderParams;
use brotli::enc::ZopfliNode;
use brotli::enc::backward_references::BrotliEncoderMode;
use brotli::enc::backward_references::BrotliHasherParams;
use brotli::enc::backward_references_hq::BrotliZopfliCreateCommands;
use brotli::enc::command::BrotliDistanceParams;

fn main() {
    BrotliZopfliCreateCommands(0usize, 0usize, 0usize, &[ZopfliNode::default()], &mut vec![], &mut 0usize, &BrotliEncoderParams { dist: BrotliDistanceParams { distance_postfix_bits: 0u32, num_direct_distance_codes: 0u32, alphabet_size: 0u32, max_distance: 1usize }, mode: BrotliEncoderMode::BROTLI_FORCE_MSB_PRIOR, quality: 0i32, q9_5: false, lgwin: 0i32, lgblock: 0i32, size_hint: 0usize, disable_literal_context_modeling: 0i32, hasher: BrotliHasherParams { type_: 0i32, bucket_bits: 0i32, block_bits: 0i32, hash_len: 0i32, num_last_distances_to_check: 0i32, literal_byte_score: 0i32 }, log_meta_block: false, stride_detection_quality: 0u8, high_entropy_detection_quality: 0u8, cdf_adaptation_detection: 1u8, prior_bitmask_detection: 0u8, literal_adaptation: [(0u16, 0u16), (0u16, 0u16), (0u16, 0u16), (1u16, 0u16)], large_window: false, avoid_distance_prefix_search: false, byte_align: false, bare_stream: false, catable: false, use_dictionary: false, appendable: false, magic_number: false, favor_cpu_efficiency: false }, &mut vec![], &mut 0usize);
}
