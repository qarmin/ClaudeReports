use image::DynamicImage;
use image::Limits;
use image::codecs::avif::ColorSpace;
use image::codecs::png::CompressionType;
use image::codecs::png::FilterType;
use image::codecs::pnm::SampleEncoding;

fn main() {
    let _ = ColorSpace::Srgb;
    let _ = SampleEncoding::Ascii;
    let dynamic_image = DynamicImage::new_luma_a32f(65535u32, 1000u32);
    let dynamic_image2 = dynamic_image.adjust_contrast(0.0f32);
    let _ = dynamic_image2.adjust_contrast(0.0f32);
    let _ = <image::codecs::png::PngEncoder<Vec<u8>>>::new_with_quality(include_bytes!("image_0.png").to_vec(), CompressionType::Level(0u8), FilterType::Up);
    let _ = <image::ImageBuffer<image::Rgba<u16>, Vec<u16>> as image::GenericImageView>::in_bounds(&dynamic_image.to_rgba16(), 2303440273u32, 7u32);
    let _ = Limits::default();
}
