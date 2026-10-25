use egui::Visuals;
use epaint::ColorImage;
use epaint::ImageData;
use epaint::TextureManager;
use epaint::textures::TextureOptions;

fn main() {
    let visuals = Visuals::default();
    let color32 = visuals.text_color();
    let texture_manager = TextureManager::default();
    let _ = texture_manager.meta(TextureManager::default().alloc("".to_string(), ImageData::Color(std::sync::Arc::new(ColorImage::filled([1usize, 1000000000usize], color32))), TextureOptions::default()));
}
