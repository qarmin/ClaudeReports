use printpdf::font::BuiltinFont;
use printpdf::font::ParsedFont;

fn main() {
    let _ = printpdf::font::get_normalized_widths_ttf(&ParsedFont::from_azul(BuiltinFont::default().get_parsed_font().unwrap().into_inner()), &[(u16::MAX, "".to_string()), (0u16, "".to_string())]);
}
