use brotli::enc::compress_fragment_two_pass::BrotliWriteBits;

fn main() {
    BrotliWriteBits(0usize, 0u64, &mut 1000000000000000000usize, &mut vec![]);
}
