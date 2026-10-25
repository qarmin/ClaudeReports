use miniz_oxide::inflate::core::BlockBoundaryState;
use miniz_oxide::inflate::core::DecompressorOxide;

fn main() {
    let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(&BlockBoundaryState::default());
    let _ = { let r = &mut decompressor_oxide; let in_buf = &vec![82u8, 73u8, 70u8, 70u8]; let out = &mut vec![0u8, 0u8, 0u8, 0u8]; let out_pos = 0usize; let flags = 0u32; miniz_oxide::inflate::core::decompress(r, in_buf, out, out_pos, flags).0 };
    let _ = miniz_oxide::inflate::core::decompress_with_limit(&mut decompressor_oxide, &vec![], &mut vec![35u8, 32u8, 84u8, 72u8, 73u8], 1usize, 5usize, u32::MAX);
}
