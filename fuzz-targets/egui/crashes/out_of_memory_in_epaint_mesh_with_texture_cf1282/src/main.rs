use ecolor::Color32;
use epaint::ColorImage;
use epaint::ImageData;
use epaint::Mesh;
use epaint::TextureManager;
use epaint::textures::TextureOptions;

fn main() {
    let _ = Mesh::with_texture(TextureManager::default().alloc("".to_string(), ImageData::from(ColorImage::filled([10000usize, 65536usize], Color32::CYAN)), TextureOptions::NEAREST_REPEAT));
}
