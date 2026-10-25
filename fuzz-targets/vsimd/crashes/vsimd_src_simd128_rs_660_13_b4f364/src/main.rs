fn main() {
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD256>::u16x16_bswap(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), <vsimd::isa::SSE41 as vsimd::SIMD256>::u64x4_splat(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), 0u64));
}
