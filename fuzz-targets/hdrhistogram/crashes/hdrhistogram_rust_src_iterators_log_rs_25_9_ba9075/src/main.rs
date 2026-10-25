fn main() {
    let histogram = <hdrhistogram::Histogram<u8>>::new(1u8).unwrap();
    let _ = histogram.iter_log(0u64, 0.0f64);
}
