use image::DynamicImage;
use image::imageops::FilterType;

fn main() {
    let _ = image::imageops::resize::<image::DynamicImage>(&DynamicImage::new_rgb8(1u32, 1u32), 1000000000u32, 0u32, FilterType::Nearest);
}
