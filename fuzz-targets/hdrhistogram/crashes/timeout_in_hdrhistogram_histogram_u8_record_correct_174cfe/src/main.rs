use hdrhistogram::errors::UsizeTypeTooSmall;

fn main() {
    let _ = UsizeTypeTooSmall;
    let mut histogram = <hdrhistogram::Histogram<u8>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_with_bounds(7u64, 13826826444047866644u64, 1u8).unwrap()));
    let _ = histogram.record_correct(u64::MAX, 4294967296u64);
}
