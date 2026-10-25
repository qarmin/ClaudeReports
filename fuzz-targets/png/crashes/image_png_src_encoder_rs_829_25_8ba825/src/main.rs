use png::ColorType;
use png::Info;

fn main() {
    let mut encoder = <png::Encoder<Vec<u8>>>::with_info(vec![], Info::with_size(u32::MAX, 1874738817u32)).unwrap();
    encoder.set_color(ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();
    let _ = writer.write_image_data(&vec![]);
}
