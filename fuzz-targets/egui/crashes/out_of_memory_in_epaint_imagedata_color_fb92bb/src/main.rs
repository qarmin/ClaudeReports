use ecolor::Color32;
use epaint::ColorImage;
use epaint::ImageData;

fn main() {
    let _ = ImageData::Color(std::sync::Arc::new(ColorImage::filled([1000000000usize, 1usize], Color32::BLACK)));
}
