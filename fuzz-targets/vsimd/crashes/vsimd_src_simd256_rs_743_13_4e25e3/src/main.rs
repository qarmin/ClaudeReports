fn main() {
    let _ = <vsimd::isa::SSE41 as vsimd::SIMD256>::u64x4_permute::<0>(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), <vsimd::isa::SSE41 as vsimd::SIMD256>::u64x4_splat(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), 0u64));
}
