fn main() {
    let _ = <vsimd::isa::SSE41 as vsimd::SIMD256>::u8x32_reduce_min(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), <vsimd::isa::SSE41 as vsimd::SIMD256>::i16x16_splat(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), 0i16));
}
