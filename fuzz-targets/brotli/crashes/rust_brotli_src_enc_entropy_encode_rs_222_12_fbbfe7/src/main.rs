use brotli::enc::entropy_encode::BrotliOptimizeHuffmanCountsForRle;

fn main() {
    BrotliOptimizeHuffmanCountsForRle(usize::MAX, &mut vec![2u32, 0u32, 65535u32], &mut include_bytes!("image_0.tiff").to_vec());
}
