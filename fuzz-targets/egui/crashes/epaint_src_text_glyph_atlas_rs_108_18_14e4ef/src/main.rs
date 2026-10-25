use ecolor::Color32;
use emath::Align;
use epaint::FontId;
use epaint::Fonts;
use epaint::Stroke;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::LayoutJob;
use epaint::text::TextFormat;
use epaint::text::VariationCoords;
use epaint::text::ViewportKey;

fn main() {
    let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::default());
    let _ = fonts.layout_uncached_for_viewport(1.0f32, ViewportKey::default(), std::sync::Arc::new(LayoutJob::simple_format("Za".to_string(), TextFormat { font_id: FontId::monospace(1.0f32), extra_letter_spacing: f32::NEG_INFINITY, line_height: None, color: Color32::from_black_alpha(0u8), background: Color32::PURPLE, expand_bg: 0.0f32, coords: VariationCoords::default(), italics: false, underline: Stroke::new(0.0f32, Color32::TRANSPARENT), strikethrough: Stroke { width: 0.0f32, color: Color32::TRANSPARENT }, valign: Align::Min })));
}
