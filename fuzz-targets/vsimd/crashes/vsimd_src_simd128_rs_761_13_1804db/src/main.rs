fn main() {
    let _ = <vsimd::isa::SSE41 as vsimd::SIMD256>::u8x32_reduce_max(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), <vsimd::isa::SSSE3 as vsimd::SIMD256>::u64x4_splat(vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap(), 0u64));
}
