use epaint::ColorImage;
use epaint::ImageData;

fn main() {
    let _ = ImageData::Color(std::sync::Arc::new(ColorImage::from_rgb([usize::MAX, 10usize], &include_bytes!("image_0.bmp").to_vec())));
}
