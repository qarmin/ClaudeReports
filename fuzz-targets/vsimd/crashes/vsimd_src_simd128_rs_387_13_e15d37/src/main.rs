use vsimd::hex::ENCODE_LOWER_LUT;
use vsimd::vector::V256;

fn main() {
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD256>::i32x8_mul_lo(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), V256::from_bytes([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]), ENCODE_LOWER_LUT);
}
