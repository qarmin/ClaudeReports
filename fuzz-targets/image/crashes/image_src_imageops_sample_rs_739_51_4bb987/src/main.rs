use image::DynamicImage;

fn main() {
    let _ = DynamicImage::new_luma_a8(10u32, 10000u32).thumbnail(0u32, 0u32);
}
