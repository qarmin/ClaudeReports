use brotli::enc::entropy_encode::BrotliWriteHuffmanTree;

fn main() {
    BrotliWriteHuffmanTree(&vec![35u8], 1usize, &mut 0usize, &mut vec![], &mut vec![]);
}
