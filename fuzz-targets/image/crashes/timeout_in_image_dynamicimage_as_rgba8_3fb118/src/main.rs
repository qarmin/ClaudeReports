use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_rgb8(65535u32, 38u32).blur(16777216.0f32);
    let _ = dynamic_image.as_rgba8();
}
