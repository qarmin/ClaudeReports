use image::DynamicImage;
use image::imageops::GaussianBlurParameters;

fn main() {
    let dynamic_image = DynamicImage::new_luma16(1u32, 53293975u32);
    let _ = dynamic_image.blur_advanced(GaussianBlurParameters::new_from_kernel_size(34.0f32));
}
