use image::DynamicImage;
use image::imageops::FilterType;

fn main() {
    let mut dynamic_image = DynamicImage::from(DynamicImage::new_luma16(0u32, 0u32));
    dynamic_image.resize_to_fill(0u32, 1u32, FilterType::Triangle);
}
