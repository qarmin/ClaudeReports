use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::ImageRgba32F(DynamicImage::new_luma16(1u32, 1u32).to_rgba32f());
    let dynamic_image2 = dynamic_image.filter3x3(&[0.0f32, 0.0f32, f32::NAN, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32]);
    let _ = dynamic_image2.brighten(0i32);
}
