use image::ExtendedColorType;

fn main() {
    let _ = <image::codecs::gif::GifEncoder<Vec<u8>> as image::ImageEncoder>::write_image(<image::codecs::gif::GifEncoder<Vec<u8>>>::new(vec![]), &vec![], 1u32, 1u32, ExtendedColorType::Rgb8);
}
