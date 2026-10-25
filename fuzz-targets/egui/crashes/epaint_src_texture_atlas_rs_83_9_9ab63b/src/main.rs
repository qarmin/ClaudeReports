use epaint::FontColorTransferFunction;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;

fn main() {
    let _ = Fonts::new(TextOptions { max_texture_side: 0usize, color_transfer_function: FontColorTransferFunction::Gamma(0.0f32), font_hinting: false, subpixel_binning: false }, FontDefinitions::empty());
}
