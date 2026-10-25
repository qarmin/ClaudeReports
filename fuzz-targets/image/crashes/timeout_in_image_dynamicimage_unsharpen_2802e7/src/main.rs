use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_rgba8(65536u32, 8u32);
    let _ = dynamic_image.unsharpen(256.0f32, 0i32);
}
