use emath::Align;
use epaint::FontColorTransferFunction;
use epaint::Fonts;
use epaint::TextOptions;
use epaint::text::FontDefinitions;
use epaint::text::LayoutJob;
use epaint::text::TextWrapping;

fn main() {
    let _ = Fonts::new(TextOptions { max_texture_side: 1000000000usize, color_transfer_function: FontColorTransferFunction::DARK_MODE_DEFAULT, font_hinting: false, subpixel_binning: false }, FontDefinitions::default()).layout_uncached(0.0f32, std::sync::Arc::new(LayoutJob { text: "a".to_string(), sections: vec![], wrap: TextWrapping { max_width: 0.0f32, max_rows: 0usize, break_anywhere: false, overflow_character: None }, first_row_min_height: 0.0f32, break_on_newline: false, halign: Align::Min, justify: false, round_output_to_gui: false, keep_trailing_whitespace: false }));
}
