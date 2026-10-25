use image::DynamicImage;

fn main() {
    let _ = DynamicImage::ImageRgb16(DynamicImage::new_rgb8(97u32, 65535u32).unsharpen(291.0f32, 0i32).to_rgb16());
}
