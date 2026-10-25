use epaint::ColorImage;
use epaint::ImageData;
use epaint::ImageDelta;
use epaint::TextureManager;
use epaint::textures::TextureOptions;

fn main() {
    let mut texture_manager = TextureManager::default();
    texture_manager.set(TextureManager::default().alloc("".to_string(), ImageData::Color(std::sync::Arc::new(ColorImage::example())), TextureOptions::NEAREST_REPEAT), ImageDelta { image: ImageData::from(ColorImage::default()), options: TextureOptions::NEAREST_MIRRORED_REPEAT, pos: None });
}
