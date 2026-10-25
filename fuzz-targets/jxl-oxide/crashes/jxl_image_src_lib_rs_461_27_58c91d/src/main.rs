use jxl_image::BitDepth;

fn main() {
    let bit_depth = BitDepth::IntegerSample { bits_per_sample: 1000000000u32 };
    let _ = bit_depth.parse_integer_sample(0i32);
}
