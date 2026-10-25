fn main() {
    let s_s_e41 = vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap();
    let s_s_e2 = vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap();
    let s_s_s_e3 = vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap();
    let _ = vsimd::hex::encode_bytes32::<vsimd::isa::SSE2>(s_s_e2, <vsimd::isa::SSSE3 as vsimd::SIMD256>::v256_create_zero(s_s_s_e3), <vsimd::isa::SSE41 as vsimd::SIMD256>::i32x8_splat(s_s_e41, 0i32));
}
