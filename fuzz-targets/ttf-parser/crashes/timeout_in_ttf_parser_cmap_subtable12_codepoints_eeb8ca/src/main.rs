use ttf_parser::cmap::Subtable12;

fn main() {
    let vec = include_bytes!("data_0.bin").to_vec();
    let subtable12 = Subtable12::parse(&vec).unwrap();
    subtable12.codepoints(|_| ());
}
