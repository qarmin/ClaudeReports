fn main() {
    let histogram = <hdrhistogram::Histogram<u64>>::new(0u8).unwrap();
    let _ = histogram.iter_quantiles(0u32);
}
