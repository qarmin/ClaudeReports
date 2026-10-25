use ttf_parser::cmap::Subtable13;

fn main() {
    let vec = include_bytes!("data_0.bin").to_vec();
    let subtable13 = Subtable13::parse(&vec).unwrap();
    subtable13.codepoints(|_| ());
}
