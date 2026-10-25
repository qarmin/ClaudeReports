use rawler::formats::tiff::CompressionMethod;
use rawler::formats::tiff::Value;

fn main() {
    let value = Value::from(CompressionMethod::Huffman);
    let _ = value.get_data();
}
