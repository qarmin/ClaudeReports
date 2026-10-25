use ecolor::Color32;
use emath::Vec2;
use epaint::ColorImage;
use epaint::text::GlyphBitmap;
use epaint::text::RasterizedGlyph;

fn main() {
    let _ = RasterizedGlyph { bitmap: GlyphBitmap { image: ColorImage::filled([1usize, 10000000000000000usize], Color32::from_white_alpha(0u8)), offset_px: Vec2::UP, is_color: false }, advance_px: 0.0f32 };
}
