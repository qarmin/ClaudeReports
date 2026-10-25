use image::DynamicImage;

fn main() {
    let _ = <image::ImageBuffer<image::Rgb<u16>, Vec<u16>> as image::GenericImageView>::buffer_like(&DynamicImage::new_rgba16(255u32, 65535u32).blur(92.0f32).into_rgb16());
}
