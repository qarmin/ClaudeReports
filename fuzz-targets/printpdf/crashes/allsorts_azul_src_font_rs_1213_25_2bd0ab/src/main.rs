use printpdf::font::ParsedFont;

fn main() {
    let _ = ParsedFont::from_bytes(&include_bytes!("data_0.bin").to_vec(), 1usize, &mut vec![]);
}
