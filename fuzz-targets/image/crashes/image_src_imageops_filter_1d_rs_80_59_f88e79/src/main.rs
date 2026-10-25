use image::DynamicImage;
use image::imageops::GaussianBlurParameters;
use image::math::Rect;

fn main() {
    let dynamic_image = DynamicImage::new_rgb32f(10u32, 0u32);
    let dynamic_image2 = dynamic_image.crop(Rect { x: 1u32, y: 0u32, width: 1u32, height: 1u32 });
    let _ = dynamic_image2.blur_advanced(GaussianBlurParameters::SMOOTHING_3);
}
