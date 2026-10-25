use vsimd::hex::ENCODE_LOWER_LUT;

fn main() {
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD256>::u8x32_blendv(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), ENCODE_LOWER_LUT, ENCODE_LOWER_LUT, ENCODE_LOWER_LUT);
}
