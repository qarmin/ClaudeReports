use ecolor::Color32;
use epaint::FontId;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::LayoutJob;
use epaint::text::ViewportKey;

fn main() {
    let _ = Fonts::new(TextOptions::default(), FontDefinitions::default()).layout_uncached_for_viewport(1.0f32, ViewportKey::default(), std::sync::Arc::new(LayoutJob::simple_singleline("a".to_string(), FontId::monospace(0.0f32), Color32::ORANGE)));
}
