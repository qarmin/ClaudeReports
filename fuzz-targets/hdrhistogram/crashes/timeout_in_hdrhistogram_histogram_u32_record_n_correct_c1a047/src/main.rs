fn main() {
    let histogram = <hdrhistogram::Histogram<u32>>::new_from::<u8>(&<hdrhistogram::Histogram<u8>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_from::<u8>(&<hdrhistogram::Histogram<u8>>::new(1u8).unwrap())));
    let histogram2 = histogram.clone_correct(0u64);
    let mut histogram3 = histogram2.clone_correct(0u64);
    let _ = histogram3.record_n_correct(u64::MAX, 255u32, 255u64);
}
