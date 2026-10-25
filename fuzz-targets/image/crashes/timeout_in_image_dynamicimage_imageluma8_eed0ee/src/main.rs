use image::DynamicImage;

fn main() {
    let _ = DynamicImage::ImageLuma8(DynamicImage::new_rgba32f(1u32, 1u32).fast_blur(1.84e16f32).into_luma8());
}
