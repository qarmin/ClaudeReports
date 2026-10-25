use image::DynamicImage;

fn main() {
    let mut dynamic_image = DynamicImage::new_luma_a8(1u32, 433870799u32);
    image::imageops::brighten_in_place::<image::DynamicImage>(&mut dynamic_image, 0i32);
}
