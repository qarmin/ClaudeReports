fn main() {
    let _ = <vsimd::isa::SSSE3 as vsimd::SIMD256>::u64x4_bswap(vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap(), <vsimd::isa::SSSE3 as vsimd::SIMD256>::u8x32_splat(vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap(), 0u8));
}
