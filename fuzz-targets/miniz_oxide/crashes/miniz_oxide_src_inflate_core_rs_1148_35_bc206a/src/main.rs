use miniz_oxide::inflate::core::BlockBoundaryState;
use miniz_oxide::inflate::core::DecompressorOxide;
use miniz_oxide::inflate::core::inflate_flags::TINFL_FLAG_PARSE_ZLIB_HEADER;

fn main() {
    let u32_v = TINFL_FLAG_PARSE_ZLIB_HEADER;
    let block_boundary_state = BlockBoundaryState::default();
    let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(&block_boundary_state);
    let _ = miniz_oxide::inflate::core::decompress(&mut decompressor_oxide, &vec![35u8, 32u8, 84u8], &mut vec![0u8, 0u8, 0u8, 0u8, 91u8, 76u8, 0u8, 0u8, 42u8, 39u8, 0u8, 0u8, 98u8, 51u8, 0u8, 0u8, 0u8, 0u8, 0u8, 58u8, 49u8, 0u8, 0u8, 0u8, 61u8, 39u8, 33u8, 0u8, 0u8, 111u8, 0u8, 69u8], 2usize, u32_v);
    let _ = miniz_oxide::inflate::core::decompress(&mut decompressor_oxide, &vec![], &mut vec![0u8], 0usize, u32::MAX);
}
