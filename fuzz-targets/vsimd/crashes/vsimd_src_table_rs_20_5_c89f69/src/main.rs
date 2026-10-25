use vsimd::hex::ENCODE_LOWER_LUT;

fn main() {
    let _ = vsimd::hex::decode_ascii32::<vsimd::isa::SSE2>(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), ENCODE_LOWER_LUT);
}
