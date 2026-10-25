fn main() {
    let mut histogram = <hdrhistogram::Histogram<u64>>::new_with_max(1000000000000000000u64, 0u8).unwrap();
    let _ = histogram.record_n_correct(4294967296u64, 1u64, 1u64);
}
