fn main() {
    let _ = <vsimd::isa::SSE41 as vsimd::SIMD128>::u8x16_unzip_odd(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), <vsimd::isa::SSE41 as vsimd::SIMD128>::i16x8_splat(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), 0i16), <vsimd::isa::SSE2 as vsimd::SIMD128>::u8x16_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0u8));
}
