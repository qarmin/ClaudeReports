use ecolor::Color32;
use epaint::ColorImage;
use epaint::ImageData;
use epaint::ImageDelta;
use epaint::textures::TextureOptions;

fn main() {
    let _ = ImageDelta::partial([1usize, 0usize], ImageData::from(std::sync::Arc::new(ColorImage::filled([1usize, 1000000000usize], Color32::YELLOW))), TextureOptions::LINEAR_MIRRORED_REPEAT);
}
