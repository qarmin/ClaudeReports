use image::Delay;
use image::DynamicImage;
use image::Frame;

fn main() {
    let _ = Frame::from_parts(DynamicImage::new_rgba8(0u32, 2u32).blur(16777216.0f32).to_rgba8(), 0u32, u32::MAX, Frame::from_parts(<image::ImageBuffer<image::Rgba<u8>, Vec<u8>> as ::core::default::Default>::default(), 65535u32, 256u32, Delay::from_millis(84u32)).delay());
}
