use ecolor::Color32;
use epaint::ColorImage;
use epaint::ImageData;
use epaint::TextureManager;
use epaint::textures::TextureOptions;

fn main() {
    let mut texture_manager = TextureManager::default();
    let _ = texture_manager.alloc("".to_string(), ImageData::from(ColorImage::filled([1000000000usize, 1usize], Color32::DEBUG_COLOR)), TextureOptions::LINEAR_REPEAT);
}
