use brotli::enc::brotli_bit_stream::BrotliStoreHuffmanTree;
use brotli::enc::entropy_encode::HuffmanTree;

fn main() {
    BrotliStoreHuffmanTree(&vec![], 0usize, &mut vec![HuffmanTree::default(), HuffmanTree::new(0u32, 0i16, 0i16)], &mut 0usize, &mut vec![]);
}
