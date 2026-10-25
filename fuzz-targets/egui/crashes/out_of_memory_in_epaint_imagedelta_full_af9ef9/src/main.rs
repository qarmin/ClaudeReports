use ecolor::Color32;
use epaint::ColorImage;
use epaint::ImageData;
use epaint::ImageDelta;
use epaint::textures::TextureOptions;

fn main() {
    let _ = ImageDelta::full(ImageData::from(std::sync::Arc::new(ColorImage::filled([1usize, 1000000000usize], Color32::MAGENTA))), TextureOptions::NEAREST);
}
