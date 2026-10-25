use image::DynamicImage;
use image::Frame;

fn main() {
    let _ = Frame::new(DynamicImage::new_luma32f(1u32, 1u32).fast_blur(f32::INFINITY).into_rgba8());
}
