fn main() {
    let mut histogram = <hdrhistogram::Histogram<u32> as ::core::convert::From<hdrhistogram::Histogram<u32>>>::from(<hdrhistogram::Histogram<u32>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_from::<u32>(&<hdrhistogram::Histogram<u32>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new(1u8).unwrap()))));
    let _ = histogram.record_correct(10000000000000000000u64, 2u64);
}
