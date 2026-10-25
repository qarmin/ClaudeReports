use vsimd::hex::ENCODE_LOWER_LUT;

fn main() {
    let s_s_e2 = vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap();
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD256>::u8x32_reduce_max(s_s_e2, ENCODE_LOWER_LUT);
}
