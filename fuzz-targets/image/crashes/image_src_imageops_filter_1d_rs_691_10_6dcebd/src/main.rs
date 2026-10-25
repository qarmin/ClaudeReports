use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::from(<image::ImageBuffer<image::Rgba<f32>, Vec<f32>> as ::core::default::Default>::default());
    let dynamic_image2 = dynamic_image.flipv();
    let _ = dynamic_image2.unsharpen(65536.0f32, 0i32);
}
