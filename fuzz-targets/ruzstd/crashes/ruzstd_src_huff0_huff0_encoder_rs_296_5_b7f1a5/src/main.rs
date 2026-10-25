use ruzstd::huff0::huff0_encoder::HuffmanTable;

fn main() {
    let _ = HuffmanTable::build_from_counts(&[]);
}
