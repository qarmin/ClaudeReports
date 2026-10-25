fn main() {
    let mut histogram = <hdrhistogram::Histogram<u32> as ::core::convert::From<hdrhistogram::Histogram<u32>>>::from(<hdrhistogram::Histogram<u32>>::new(1u8).unwrap());
    let _ = histogram.record(u64::MAX);
    let _ = <hdrhistogram::SyncHistogram<u32> as ::core::convert::From<hdrhistogram::Histogram<u32>>>::from(histogram.clone_correct(2u64));
}
