fn main() {
    let s_s_s_e3 = vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap();
    let _ = <vsimd::isa::SSSE3 as vsimd::SIMD256>::u32x8_bswap(vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap(), <vsimd::isa::SSSE3 as vsimd::SIMD256>::i64x4_splat(s_s_s_e3, 0i64));
}
