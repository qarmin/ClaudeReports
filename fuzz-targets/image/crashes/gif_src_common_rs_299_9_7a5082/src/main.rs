use image::ColorType;
use image::ExtendedColorType;

fn main() {
    let mut gif_encoder = <image::codecs::gif::GifEncoder<Vec<u8>>>::new(vec![]);
    let _ = gif_encoder.encode(&vec![], 1u32, 1u32, ExtendedColorType::from(ColorType::La8));
}
