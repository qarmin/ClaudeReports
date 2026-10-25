use image::DynamicImage;

fn main() {
    let _ = DynamicImage::from(DynamicImage::new_rgb8(256u32, 42u32).unsharpen(65536.0f32, 33554434i32).into_luma_alpha8());
}
