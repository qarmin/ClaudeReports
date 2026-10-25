use vsimd::hex::ENCODE_LOWER_LUT;

fn main() {
    let _ = <vsimd::isa::SSSE3 as vsimd::SIMD256>::u8x32_unzip_even(vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap(), ENCODE_LOWER_LUT, ENCODE_LOWER_LUT);
}
