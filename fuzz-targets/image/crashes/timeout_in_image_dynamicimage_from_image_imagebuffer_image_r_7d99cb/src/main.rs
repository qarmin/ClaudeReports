use image::DynamicImage;

fn main() {
    let _ = DynamicImage::from(DynamicImage::new_luma_a8(1u32, 1u32).fast_blur(-2147483600.0f32).into_rgb32f());
}
