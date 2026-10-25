fn main() {
    let histogram = <hdrhistogram::Histogram<u64>>::new_from::<u32>(&<hdrhistogram::Histogram<u32>>::new(0u8).unwrap());
    let sync_histogram = histogram.clone().into_sync();
    let mut recorder = sync_histogram.recorder();
    let _ = recorder.record_n_correct(u64::MAX, 0u64, 2u64);
}
