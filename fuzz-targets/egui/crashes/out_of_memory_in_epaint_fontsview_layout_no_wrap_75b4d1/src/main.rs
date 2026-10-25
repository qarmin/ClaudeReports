use ecolor::Color32;
use epaint::FontFamily;
use epaint::FontId;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::ViewportKey;

fn main() {
    let color32 = Color32::DARK_GRAY;
    let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
    let mut fonts_view = fonts.with_pixels_per_point_for_viewport(721.0f32, ViewportKey::default());
    let _ = fonts_view.layout_no_wrap("a".to_string(), FontId::new(642.0f32, FontFamily::Monospace), color32);
}
