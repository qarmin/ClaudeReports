use image::DynamicImage;

fn main() {
    let _ = DynamicImage::from(DynamicImage::new_luma_a8(1u32, 65535u32).fast_blur(16777216.0f32));
}
