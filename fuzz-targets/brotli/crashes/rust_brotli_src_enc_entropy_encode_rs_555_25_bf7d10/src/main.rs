use brotli::enc::entropy_encode::BrotliConvertBitDepthsToSymbols;

fn main() {
    BrotliConvertBitDepthsToSymbols(&vec![35u8], 1usize, &mut vec![]);
}
