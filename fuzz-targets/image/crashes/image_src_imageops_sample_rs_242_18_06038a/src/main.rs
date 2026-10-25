use image::DynamicImage;
use image::imageops::FilterType;

fn main() {
    let _ = image::imageops::resize::<image::DynamicImage>(&DynamicImage::new_luma_a32f(1u32, 1u32), 0u32, 1u32, FilterType::Gaussian);
}
