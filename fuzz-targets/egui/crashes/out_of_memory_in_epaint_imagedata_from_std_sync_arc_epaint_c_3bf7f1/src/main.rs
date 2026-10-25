use ecolor::Color32;
use epaint::ColorImage;
use epaint::ImageData;

fn main() {
    let _ = ImageData::from(std::sync::Arc::new(ColorImage::filled([1usize, 1000000000usize], Color32::DARK_BLUE)));
}
