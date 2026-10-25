use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_luma_a32f(1u32, 1u32);
    let dynamic_image2 = dynamic_image.grayscale();
    let dynamic_image3 = dynamic_image2.adjust_contrast(0.0f32);
    let _ = dynamic_image3.fast_blur(f32::NEG_INFINITY);
}
