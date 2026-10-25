use ecolor::Color32;
use epaint::FontId;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::LayoutJob;
use epaint::text::ViewportKey;

fn main() {
    let mut fonts = Fonts::new(TextOptions::default(), FontDefinitions::default());
    let _ = fonts.layout_uncached_for_viewport(10000000000.0f32, ViewportKey::default(), std::sync::Arc::new(LayoutJob::simple_singleline("\"".to_string(), FontId::default(), Color32::from_rgb_additive(0u8, 0u8, 0u8))));
}
