use image::ColorType;
use image::DynamicImage;
use image::ImageFormat;
use image::ImageLayout;
use image::codecs::png::CompressionType;
use image::codecs::png::FilterType;
use image::io::DecodedMetadataHint;
use image::io::DecoderPreparedImage;
use image::metadata::CicpMatrixCoefficients;

fn main() {
    let decoder_prepared_image = DecoderPreparedImage::from(ImageLayout::from(ImageLayout::from(ImageLayout::new(16u32, 1495293434u32, ColorType::Rgba8))));
    let _ = decoder_prepared_image.total_bytes();
    let mut png_encoder = <image::codecs::png::PngEncoder<Vec<u8>>>::new_with_quality(include_bytes!("mutated_0_V4_A8_R8_G8_B8.bmp.png").to_vec(), CompressionType::default(), FilterType::Avg);
    let _ = <image::codecs::png::PngEncoder<Vec<u8>> as image::ImageEncoder>::set_xmp_metadata(&mut png_encoder, include_bytes!("iptc.jpg.png").to_vec());
    let _ = image::load::<::std::io::Cursor<Vec<u8>>>(std::io::Cursor::new(include_bytes!("image_2.gif").to_vec()), ImageFormat::Farbfeld);
    let _ = CicpMatrixCoefficients::Smpte2085;
    let _ = DecodedMetadataHint::None;
    let dynamic_image = DynamicImage::ImageRgb8(DynamicImage::new_rgba8(16u32, 65535u32).brighten(45i32).unsharpen(1000.0f32, 65536i32).into_rgb8());
    let image_buffer = dynamic_image.to_luma_alpha32f();
    let _ = image_buffer.pixels();
}
