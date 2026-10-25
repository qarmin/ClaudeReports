use ecolor::Color32;
use emath::Rect;
use epaint::Brush;
use epaint::ColorImage;
use epaint::ImageData;
use epaint::TextureManager;
use epaint::textures::TextureOptions;

fn main() {
    let _ = Brush { fill_texture_id: TextureManager::default().alloc("".to_string(), ImageData::from(ColorImage::filled([1usize, 1000000000usize], Color32::BLACK)), TextureOptions::NEAREST_MIRRORED_REPEAT), uv: Rect::everything_below(0.0f32) };
}
