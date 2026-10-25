use vsimd::hex::ENCODE_LOWER_LUT;

fn main() {
    let _ = <vsimd::isa::SSE41 as vsimd::SIMD256>::u32x8_blend::<0>(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), ENCODE_LOWER_LUT, ENCODE_LOWER_LUT);
}
