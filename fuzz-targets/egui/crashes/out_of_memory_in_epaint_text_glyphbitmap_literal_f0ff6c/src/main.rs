use ecolor::Color32;
use epaint::ColorImage;
use epaint::text::GlyphBitmap;

fn main() {
    let _ = GlyphBitmap { image: ColorImage::filled([1usize, 1000000000usize], Color32::GOLD), offset_px: emath::vec2(0.0f32, 0.0f32), is_color: false };
}
