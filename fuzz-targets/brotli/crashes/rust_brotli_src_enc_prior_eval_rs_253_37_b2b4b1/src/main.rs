fn main() {
    let _ = <brotli::enc::prior_eval::CMPrior as brotli::enc::prior_eval::Prior>::lookup_lin(0u8, 0u8, usize::MAX, Some(15u8));
}
