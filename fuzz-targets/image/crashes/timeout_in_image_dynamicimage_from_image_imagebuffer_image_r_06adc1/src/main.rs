use image::DynamicImage;

fn main() {
    let _ = DynamicImage::from(DynamicImage::new_rgb32f(1u32, 1u32).fast_blur(-20200000000000.0f32).to_rgb8());
}
