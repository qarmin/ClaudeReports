use vsimd::hex::ENCODE_LOWER_LUT;

fn main() {
    let s_s_s_e3 = vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap();
    let _ = <vsimd::isa::SSSE3 as vsimd::SIMD256>::u16x16_max(s_s_s_e3, ENCODE_LOWER_LUT, ENCODE_LOWER_LUT);
}
