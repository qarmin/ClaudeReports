fn main() {
    let histogram = <hdrhistogram::Histogram<u32>>::new_with_max(8u64, 1u8).unwrap();
    let _ = <hdrhistogram::iterators::log::Iter<u32>>::new(&histogram, 1u64, 0.0f64);
}
