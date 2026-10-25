use png::BitDepth;
use png::ColorType;
use png::OutputInfo;

fn main() {
    let output_info = OutputInfo { width: 0u32, height: 100000000u32, color_type: ColorType::Rgba, bit_depth: BitDepth::from_u8(1u8).unwrap(), line_size: 10000000000000000000usize };
    let _ = output_info.buffer_size();
}
