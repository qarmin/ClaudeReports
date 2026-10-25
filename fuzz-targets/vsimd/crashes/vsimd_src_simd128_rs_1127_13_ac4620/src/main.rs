fn main() {
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD128>::i16x8_maddubs(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), <vsimd::isa::SSE2 as vsimd::SIMD128>::i64x2_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0i64), <vsimd::isa::SSE2 as vsimd::SIMD128>::i8x16_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0i8));
}
