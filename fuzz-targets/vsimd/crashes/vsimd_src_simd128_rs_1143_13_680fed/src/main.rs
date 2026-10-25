fn main() {
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD128>::u16x8_blend::<0>(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), <vsimd::isa::SSE2 as vsimd::SIMD128>::u32x4_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0u32), <vsimd::isa::SSE41 as vsimd::SIMD128>::i64x2_splat(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), 0i64));
}
