use miniz_oxide::inflate::core::BlockBoundaryState;
use miniz_oxide::inflate::core::DecompressorOxide;

fn main() {
    let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(&BlockBoundaryState::default());
    let _ = miniz_oxide::inflate::core::decompress(&mut decompressor_oxide, &vec![35u8, 32u8, 84u8], &mut vec![0u8, 68u8, 78u8, 0u8, 66u8, 0u8, 126u8, 0u8, 0u8, 32u8, 0u8, 0u8, 0u8, 0u8, 0u8, 77u8, 0u8, 0u8, 75u8, 0u8, 0u8, 0u8, 104u8, 76u8, 0u8, 113u8, 0u8, 60u8, 99u8, 82u8, 42u8, 0u8], 3usize, 1u32);
    let _ = miniz_oxide::inflate::core::decompress_with_limit(&mut decompressor_oxide, &vec![], &mut vec![82u8, 73u8], 0usize, 1000000000usize, 6u32);
}
