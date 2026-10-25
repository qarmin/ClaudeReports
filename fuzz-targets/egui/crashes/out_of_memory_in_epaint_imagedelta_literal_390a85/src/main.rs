use ecolor::Color32;
use epaint::ColorImage;
use epaint::ImageData;
use epaint::ImageDelta;
use epaint::textures::TextureOptions;

fn main() {
    let _ = ImageDelta { image: ImageData::Color(std::sync::Arc::new(ColorImage::filled([1usize, 10000000000000usize], Color32::DARK_RED))), options: TextureOptions::LINEAR, pos: None };
}
