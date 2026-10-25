use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::from(<image::ImageBuffer<image::Rgb<f32>, Vec<f32>>>::from_raw(1u32, 0u32, vec![]).unwrap());
    let _ = dynamic_image.blur(409.0f32);
}
