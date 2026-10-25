use image::DynamicImage;

fn main() {
    let _ = DynamicImage::new_luma8(1u32, 1u32).adjust_contrast(f32::NAN);
}
