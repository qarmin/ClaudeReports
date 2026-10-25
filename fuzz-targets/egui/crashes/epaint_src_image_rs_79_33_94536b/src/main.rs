use egui::Context;
use egui::Style;
use egui::Theme;
use egui_extras::syntax_highlighting::CodeTheme;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;

fn main() {
    let _ = Fonts::new(TextOptions::default(), FontDefinitions::empty()).layout_uncached(35184372000000.0f32, std::sync::Arc::new(egui_extras::syntax_highlighting::highlight(&Context::default(), &Theme::Light.default_style(), &CodeTheme::from_memory(&Context::default(), &Style::default()), "Z", "")));
}
