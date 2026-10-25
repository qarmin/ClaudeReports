use image::DynamicImage;
use image::imageops::GaussianBlurParameters;

fn main() {
    let dynamic_image = DynamicImage::ImageLuma32F(<image::ImageBuffer<image::Luma<f32>, Vec<f32>>>::from_raw(0u32, 0u32, vec![0.0f32]).unwrap());
    let _ = dynamic_image.blur_advanced(GaussianBlurParameters::SMOOTHING_7);
}
