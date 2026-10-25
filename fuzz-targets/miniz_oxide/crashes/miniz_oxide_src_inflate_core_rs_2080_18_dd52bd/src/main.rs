use miniz_oxide::inflate::core::BlockBoundaryState;
use miniz_oxide::inflate::core::DecompressorOxide;

fn main() {
    let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(&BlockBoundaryState { num_bits: 100u8, bit_buf: 0u8, z_header0: 0u32, z_header1: 0u32, check_adler32: 0u32 });
    let _ = miniz_oxide::inflate::core::decompress(&mut decompressor_oxide, &vec![], &mut vec![], 0usize, 0u32);
}
