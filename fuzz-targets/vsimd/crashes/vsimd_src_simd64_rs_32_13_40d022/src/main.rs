use vsimd::vector::V64;

fn main() {
    let _ = <vsimd::isa::SSE41 as vsimd::SIMD64>::u8x8_unzip_even(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), vsimd::hex::sse2::merge_bits(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), <vsimd::isa::SSE41 as vsimd::SIMD128>::u64x2_splat(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), 0u64)), V64::from_bytes([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]));
}
