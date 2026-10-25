fn main() {
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD128>::v128_all_zero(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), <vsimd::isa::SSE2 as vsimd::SIMD128>::i32x4_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0i32));
}
