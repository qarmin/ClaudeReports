use egui::FontSelection;
use egui::Style;
use egui::TextStyle;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::MissingGlyphPolicy;

fn main() {
    let fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
    let mut fonts2 = fonts.with_missing_glyph_policy(MissingGlyphPolicy::default());
    let mut fonts_view = fonts2.with_pixels_per_point(16777216.0f32);
    let _ = fonts_view.layout_delayed_color("a".to_string(), FontSelection::default().resolve_with_fallback(&Style::default(), FontSelection::Style(TextStyle::Body)), 0.0f32);
}
