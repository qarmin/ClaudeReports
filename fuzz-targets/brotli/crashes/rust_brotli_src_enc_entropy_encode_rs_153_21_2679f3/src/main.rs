use brotli::enc::entropy_encode::BrotliCreateHuffmanTree;

fn main() {
    BrotliCreateHuffmanTree(&[0u32, 1u32], 2usize, 0i32, &mut vec![], &mut vec![]);
}
