fn main() {
    let _ = <vsimd::isa::SSSE3 as vsimd::SIMD128>::u8x16_reduce_min(vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap(), <vsimd::isa::SSE2 as vsimd::SIMD128>::i16x8_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0i16));
}
