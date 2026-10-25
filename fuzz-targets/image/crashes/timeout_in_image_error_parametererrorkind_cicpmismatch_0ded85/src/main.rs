use image::DynamicImage;
use image::codecs::pnm::SampleEncoding;
use image::error::ParameterErrorKind;

fn main() {
    let _ = SampleEncoding::Binary;
    let _ = <image::Rgb<u16> as image::Pixel>::to_luma_alpha(&<image::Rgb<u16> as image::Pixel>::broadcast(1000u16));
    let dynamic_image = DynamicImage::ImageRgb32F(DynamicImage::new_luma_a16(16777216u32, 1u32).to_rgb32f());
    let _ = ParameterErrorKind::CicpMismatch { expected: dynamic_image.fast_blur(-42.5f32).into_luma8().color_space(), found: <image::ImageBuffer<image::Rgba<u16>, Vec<u16>> as ::core::default::Default>::default().color_space() };
}
