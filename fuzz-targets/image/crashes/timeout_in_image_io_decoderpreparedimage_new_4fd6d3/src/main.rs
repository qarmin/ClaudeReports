use image::DynamicImage;
use image::io::DecoderPreparedImage;

fn main() {
    let _ = DecoderPreparedImage::new(0u32, 1u32, DynamicImage::new_rgb8(1u32, 1u32).fast_blur(f32::MAX).color());
}
