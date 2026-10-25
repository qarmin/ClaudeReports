use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_rgba8(2u32, 8u32);
    let _ = dynamic_image.blur(16777216.0f32);
}
