use egui::Context;
use egui::Style;
use egui_extras::syntax_highlighting::CodeTheme;
use epaint::FontColorTransferFunction;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::stats::AllocInfo;
use epaint::text::FontDefinitions;

fn main() {
    let alloc_info = AllocInfo::from_galley(&Fonts::new(TextOptions { max_texture_side: 1000000000usize, color_transfer_function: FontColorTransferFunction::default(), font_hinting: false, subpixel_binning: false }, FontDefinitions::empty()).layout_uncached(1.0f32, std::sync::Arc::new(egui_extras::syntax_highlighting::highlight(&Context::default(), &Style::default(), &CodeTheme::from_style(&Style::default()), "", ""))));
    let _ = alloc_info.num_elements();
}
