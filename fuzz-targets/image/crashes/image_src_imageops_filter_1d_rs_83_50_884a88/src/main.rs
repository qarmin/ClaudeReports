use image::DynamicImage;
use image::imageops::GaussianBlurParameters;

fn main() {
    let _ = DynamicImage::default().blur_advanced(GaussianBlurParameters::SMOOTHING_5);
}
