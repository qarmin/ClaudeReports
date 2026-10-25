fn main() {
    let s_s_e2 = vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap();
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD256>::u8x32_unzip_odd(s_s_e2, <vsimd::isa::SSE41 as vsimd::SIMD256>::u8x32_splat(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), 0u8), <vsimd::isa::SSSE3 as vsimd::SIMD256>::u32x8_splat(vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap(), 0u32));
}
