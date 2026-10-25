use image::DynamicImage;

fn main() {
    let dynamic_image = DynamicImage::from(DynamicImage::new_luma8(1u32, 44917274u32).into_rgba16());
    let _ = dynamic_image.thumbnail(0u32, 1u32);
}
