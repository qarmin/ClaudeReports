use image::DynamicImage;

fn main() {
    let mut dynamic_image = DynamicImage::ImageLumaA8(<image::ImageBuffer<image::LumaA<u8>, Vec<u8>>>::from_raw(1u32, 1u32, vec![77u8, 54u8]).unwrap());
    image::imageops::brighten_in_place::<image::DynamicImage>(&mut dynamic_image, i32::MAX);
}
