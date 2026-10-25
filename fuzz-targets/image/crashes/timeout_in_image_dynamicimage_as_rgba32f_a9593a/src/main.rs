use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_luma16(1u32, 1u32).fast_blur(f32::NEG_INFINITY);
    let _ = dynamic_image.as_rgba32f();
}
