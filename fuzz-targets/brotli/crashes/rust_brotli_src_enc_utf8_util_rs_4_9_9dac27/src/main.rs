use brotli::enc::literal_cost::BrotliEstimateBitCostsForLiterals;

fn main() {
    BrotliEstimateBitCostsForLiterals(0usize, 1usize, 1usize, &vec![], &mut vec![]);
}
