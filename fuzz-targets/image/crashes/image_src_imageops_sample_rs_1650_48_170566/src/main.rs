use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_luma32f(1u32, 1u32);
    let dynamic_image2 = dynamic_image.filter3x3(&[0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, f32::INFINITY, 0.0f32, 0.0f32, 0.0f32]);
    let _ = dynamic_image2.unsharpen(0.0f32, 0i32);
}
