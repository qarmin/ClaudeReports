use ecolor::Color32;
use epaint::FontId;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;

fn main() {
    let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
    let mut fonts_view = fonts.with_pixels_per_point(65536.0f32);
    let _ = fonts_view.layout("a".to_string(), FontId::default(), Color32::LIGHT_YELLOW, 0.0f32);
}
