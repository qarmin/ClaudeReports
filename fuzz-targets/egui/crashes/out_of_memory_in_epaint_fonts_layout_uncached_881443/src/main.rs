use egui::Context;
use egui::Style;
use egui_extras::syntax_highlighting::CodeTheme;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::MissingGlyphPolicy;

fn main() {
    let fonts = Fonts::new(TextOptions::default(), FontDefinitions::empty());
    let mut fonts2 = fonts.with_missing_glyph_policy(MissingGlyphPolicy::Tofu);
    let _ = fonts2.layout_uncached(65536.0f32, std::sync::Arc::new(egui_extras::syntax_highlighting::highlight(&Context::default(), &Style::default(), &CodeTheme::default(), "C", "")));
}
