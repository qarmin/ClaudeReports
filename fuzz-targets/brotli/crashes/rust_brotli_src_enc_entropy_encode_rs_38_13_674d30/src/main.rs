use brotli::enc::entropy_encode::BrotliSetDepth;
use brotli::enc::entropy_encode::HuffmanTree;

fn main() {
    let _ = BrotliSetDepth(0i32, &mut vec![HuffmanTree::default()], &mut vec![], 1000000000i32);
}
