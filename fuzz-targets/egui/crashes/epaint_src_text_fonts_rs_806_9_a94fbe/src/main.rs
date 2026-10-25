use ecolor::Color32;
use ecolor::Rgba;
use egui::Context;
use egui::FontSelection;
use egui::Style;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::MissingGlyphPolicy;

fn main() {
    let rgba = Rgba::BLUE;
    let f32_v = rgba.intensity();
    let fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
    let fonts2 = fonts.with_missing_glyph_policy(Context::default().missing_glyph_policy());
    let mut fonts3 = fonts2.with_missing_glyph_policy(MissingGlyphPolicy::Panic);
    let mut fonts_view = fonts3.with_pixels_per_point(f32_v);
    let color32 = Color32::LIGHT_BLUE;
    let _ = fonts_view.layout("a".to_string(), FontSelection::default().resolve(&Style::default()), color32, 0.0f32);
}
