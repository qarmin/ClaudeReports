use ruzstd::huff0::HuffmanDecoder;
use ruzstd::huff0::HuffmanTable;

fn main() {
    let huffman_table = HuffmanTable::new();
    let mut huffman_decoder = HuffmanDecoder::new(&huffman_table);
    let _ = huffman_decoder.decode_symbol();
}
