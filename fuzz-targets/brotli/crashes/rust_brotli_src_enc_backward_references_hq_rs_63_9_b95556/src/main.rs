use brotli::enc::backward_references_hq::BrotliInitZopfliNodes;
use brotli::enc::constants::BROTLI_NUM_LITERAL_SYMBOLS;

fn main() {
    let usize_v = BROTLI_NUM_LITERAL_SYMBOLS;
    BrotliInitZopfliNodes(&mut vec![], usize_v);
}
