use ecolor::Color32;
use epaint::ColorImage;
use epaint::ImageData;
use epaint::Mesh16;
use epaint::TextureManager;
use epaint::textures::TextureOptions;

fn main() {
    let _ = Mesh16 { indices: vec![], vertices: vec![], texture_id: TextureManager::default().alloc("".to_string(), ImageData::from(ColorImage::filled([100000000000usize, 1usize], Color32::PLACEHOLDER)), TextureOptions::NEAREST) };
}
