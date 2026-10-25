use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_rgba32f(1u32, 1u32).fast_blur(10000000000.0f32);
    let _ = dynamic_image.as_rgba16();
}
