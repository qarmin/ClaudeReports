use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::new_rgb8(100u32, 65535u32).unsharpen(151.0f32, 0i32);
    let _ = <image::DynamicImage as image::GenericImageView>::pixels(&dynamic_image);
}
