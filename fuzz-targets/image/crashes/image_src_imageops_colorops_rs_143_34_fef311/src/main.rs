use image::DynamicImage;

fn main() {
    let mut dynamic_image = DynamicImage::new_luma8(1u32, 1u32);
    image::imageops::contrast_in_place::<image::DynamicImage>(&mut dynamic_image, f32::NAN);
}
