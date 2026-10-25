use image::DynamicImage;

fn main() {
    let _ = DynamicImage::ImageLuma32F(DynamicImage::new_rgb8(1u32, 65536u32).fast_blur(16777216.0f32).to_luma32f());
}
