use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_rgb16(255u32, 10u32).thumbnail(65535u32, 1000000000u32);
    let _ = dynamic_image.as_luma_alpha16();
}
