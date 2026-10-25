fn main() {
    let histogram = <hdrhistogram::Histogram<u8>>::new_from::<u32>(&<hdrhistogram::Histogram<u32>>::new_with_bounds(1u64, 12164973431588361683u64, 2u8).unwrap());
    let sync_histogram = histogram.clone().into_sync();
    let mut recorder = sync_histogram.recorder();
    let _ = recorder.record_correct(1000000000000000000u64, 1u64);
}
