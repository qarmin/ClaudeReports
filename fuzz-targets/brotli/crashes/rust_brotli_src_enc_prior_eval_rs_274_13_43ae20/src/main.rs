fn main() {
    let _ = <brotli::enc::prior_eval::FastCMPrior as brotli::enc::prior_eval::Prior>::lookup_lin(0u8, 0u8, 10000000000000000000usize, Some(255u8));
}
