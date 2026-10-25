use ecolor::Color32;
use egui::Style;
use egui::TextStyle;
use epaint::FontColorTransferFunction;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::LayoutJob;
use epaint::text::TextFormat;

fn main() {
    let mut fonts = Fonts::new(TextOptions { max_texture_side: 10000000000000000usize, color_transfer_function: FontColorTransferFunction::DARK_MODE_DEFAULT, font_hinting: false, subpixel_binning: false }, FontDefinitions::default());
    let _ = fonts.layout_uncached(0.0f32, std::sync::Arc::new(LayoutJob::single_section("a".to_string(), TextFormat::simple(TextStyle::Heading.resolve(&Style::default()), Color32::LIGHT_RED))));
}
