use ecolor::Color32;
use emath::Pos2;
use epaint::FontFamily;
use epaint::FontId;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::TextShape;
use epaint::text::FontDefinitions;
use epaint::text::LayoutJob;

fn main() {
    let _ = TextShape::new(Pos2 { x: 1.0f32, y: 0.0f32 }, std::sync::Arc::new(Fonts::new(TextOptions::default(), FontDefinitions::empty()).layout_uncached(65536.0f32, std::sync::Arc::new(LayoutJob::simple("a".to_string(), FontId::new(1.0f32, FontFamily::Proportional), Color32::from_rgb(0u8, 0u8, 0u8), 0.0f32)))), Color32::PURPLE);
}
