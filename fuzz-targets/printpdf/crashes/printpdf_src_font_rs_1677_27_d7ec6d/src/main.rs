use printpdf::font::BuiltinFont;

fn main() {
    let _ = printpdf::font::get_normalized_widths_codes(&BuiltinFont::default().get_parsed_font().unwrap(), &[(u16::MAX, 0u16), (0u16, 0u16)]);
}
