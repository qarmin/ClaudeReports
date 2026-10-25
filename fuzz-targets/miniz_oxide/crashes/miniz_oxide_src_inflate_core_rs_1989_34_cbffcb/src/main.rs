use miniz_oxide::inflate::core::BlockBoundaryState;
use miniz_oxide::inflate::core::DecompressorOxide;

fn main() {
    let block_boundary_state = BlockBoundaryState { num_bits: 100u8, bit_buf: 99u8, z_header0: 0u32, z_header1: 0u32, check_adler32: 0u32 };
    let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(&block_boundary_state);
    let _ = miniz_oxide::inflate::core::decompress_with_limit(&mut decompressor_oxide, &vec![], &mut vec![73u8], 0usize, 1usize, 0u32);
}
