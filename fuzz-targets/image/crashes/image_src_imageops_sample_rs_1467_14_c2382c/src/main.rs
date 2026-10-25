use image::DynamicImage;
use image::imageops::GaussianBlurParameters;

fn main() {
    let dynamic_image = DynamicImage::ImageLumaA32F(<image::ImageBuffer<image::LumaA<f32>, Vec<f32>>>::from_raw(1u32, 0u32, vec![0.0f32]).unwrap());
    let _ = dynamic_image.blur_advanced(GaussianBlurParameters::SMOOTHING_3);
}
