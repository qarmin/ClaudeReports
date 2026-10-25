use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_luma16(1u32, 0u32).blur(16777216.0f32);
    let _ = dynamic_image.as_rgb16();
}
