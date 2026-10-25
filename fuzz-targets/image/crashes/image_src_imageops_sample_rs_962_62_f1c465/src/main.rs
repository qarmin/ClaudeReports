use image::DynamicImage;

fn main() {
    let _ = DynamicImage::new_luma16(1u32, 1u32).filter3x3(&[0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, f32::NEG_INFINITY]);
}
