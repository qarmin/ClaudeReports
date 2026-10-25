use ecolor::Color32;
use epaint::ColorImage;
use epaint::ImageData;
use epaint::TextureManager;
use epaint::textures::TextureOptions;

fn main() {
    let _ = TextureManager::default().alloc("".to_string(), ImageData::from(ColorImage::filled([10000000000000000000usize, 1usize], Color32::from_rgb_additive(7u8, 192u8, 248u8))), TextureOptions::default());
}
