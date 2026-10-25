use vsimd::vector::V128;

fn main() {
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD128>::u8x16_unzip_even(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), V128::from_bytes([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]), <vsimd::isa::SSE2 as vsimd::SIMD128>::u16x8_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0u16));
}
