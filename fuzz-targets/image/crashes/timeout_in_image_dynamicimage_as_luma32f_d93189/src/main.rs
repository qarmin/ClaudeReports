use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_luma8(100u32, 65535u32).unsharpen(1000.0f32, 8i32);
    let _ = dynamic_image.as_luma32f();
}
