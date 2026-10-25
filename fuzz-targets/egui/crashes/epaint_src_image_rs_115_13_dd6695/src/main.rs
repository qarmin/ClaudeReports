use epaint::ColorImage;
use epaint::ImageData;
use epaint::TextureManager;
use epaint::textures::TextureOptions;

fn main() {
    let _ = TextureManager::default().alloc("".to_string(), ImageData::from(std::sync::Arc::new(ColorImage::from_rgba_unmultiplied([1usize, usize::MAX], &include_bytes!("image_0.webp").to_vec()))), TextureOptions::NEAREST_REPEAT);
}
