fn main() {
    let _ = <vsimd::isa::SSSE3 as vsimd::SIMD128>::u16x8_min(vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap(), <vsimd::isa::SSE2 as vsimd::SIMD128>::i64x2_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0i64), <vsimd::isa::SSE2 as vsimd::SIMD128>::i16x8_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0i16));
}
