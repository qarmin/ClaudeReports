fn main() {
    let histogram = <hdrhistogram::Histogram<u64>>::new_from::<u16>(&<hdrhistogram::Histogram<u16>>::new_from::<u32>(&<hdrhistogram::Histogram<u32>>::new_with_max(10u64, 0u8).unwrap()));
    let _ = histogram.iter_linear(0u64);
}
