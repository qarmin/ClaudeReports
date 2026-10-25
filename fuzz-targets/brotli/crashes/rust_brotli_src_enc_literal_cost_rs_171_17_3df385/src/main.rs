use brotli::enc::literal_cost::BrotliEstimateBitCostsForLiterals;

fn main() {
    BrotliEstimateBitCostsForLiterals(0usize, 1usize, 0usize, &vec![35u8], &mut vec![]);
}
