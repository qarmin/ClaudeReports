use image::DynamicImage;
use image::ImageLayout;
use image::io::DecoderPreparedImage;

fn main() {
    let _ = DecoderPreparedImage::from(ImageLayout::empty(DynamicImage::new_rgba16(1u32, 1u32).fast_blur(f32::MIN).color()));
}
