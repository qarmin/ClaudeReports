use image::DynamicImage;

fn main() {
    let _ = <image::DynamicImage as image::GenericImageView>::dimensions(&DynamicImage::from(DynamicImage::new_rgba8(1u32, 1u32).fast_blur(1.7197081e24f32).to_rgba16()));
}
