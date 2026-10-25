use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_rgba8(1u32, 1u32).fast_blur(-3882911600000000.0f32);
    let _ = dynamic_image.as_luma8();
}
