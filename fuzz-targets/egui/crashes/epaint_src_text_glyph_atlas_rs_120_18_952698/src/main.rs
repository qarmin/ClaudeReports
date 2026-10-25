use ecolor::Color32;
use epaint::FontColorTransferFunction;
use epaint::FontId;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::LayoutJob;

fn main() {
    let mut fonts = Fonts::new(TextOptions { max_texture_side: 10000usize, color_transfer_function: FontColorTransferFunction::Gamma(0.0f32), font_hinting: false, subpixel_binning: true }, FontDefinitions::default());
    let _ = fonts.layout_uncached(1e30f32, std::sync::Arc::new(LayoutJob::simple("\u{301}".to_string(), FontId::monospace(1.0f32), Color32::WHITE, 0.0f32)));
}
