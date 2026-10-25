use ecolor::Color32;
use epaint::ColorImage;
use epaint::ImageData;

fn main() {
    let _ = ImageData::from(ColorImage::filled([1000000000usize, 1usize], Color32::PURPLE));
}
