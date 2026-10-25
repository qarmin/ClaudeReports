use image::DynamicImage;
use image::metadata::CicpTransform;

fn main() {
    let _ = CicpTransform::new(<image::ImageBuffer<image::Rgb<f32>, Vec<f32>> as ::core::default::Default>::default().color_space(), DynamicImage::new_rgb16(10000u32, 1000u32).blur(1000.0f32).to_rgb32f().color_space());
}
