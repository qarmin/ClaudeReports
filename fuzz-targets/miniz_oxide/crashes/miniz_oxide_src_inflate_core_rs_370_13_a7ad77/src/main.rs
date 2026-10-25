use miniz_oxide::inflate::core::BlockBoundaryState;
use miniz_oxide::inflate::core::DecompressorOxide;

fn main() {
    let decompressor_oxide = DecompressorOxide::from_block_boundary_state(&BlockBoundaryState { num_bits: 100u8, bit_buf: 0u8, z_header0: 1u32, z_header1: 0u32, check_adler32: 1u32 });
    let _ = decompressor_oxide.block_boundary_state();
}
