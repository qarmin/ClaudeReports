use jxl_image::BitDepth;

fn main() {
    let bit_depth = BitDepth::FloatSample { bits_per_sample: 0u32, exp_bits: 0u32 };
    let _ = bit_depth.parse_integer_sample(0i32);
}
