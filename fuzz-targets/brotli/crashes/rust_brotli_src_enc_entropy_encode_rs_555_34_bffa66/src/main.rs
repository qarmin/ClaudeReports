use brotli::enc::entropy_encode::BrotliConvertBitDepthsToSymbols;

fn main() {
    BrotliConvertBitDepthsToSymbols(&vec![], 1usize, &mut vec![]);
}
