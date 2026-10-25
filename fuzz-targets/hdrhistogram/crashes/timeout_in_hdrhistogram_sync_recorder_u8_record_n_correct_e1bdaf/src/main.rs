fn main() {
    let histogram = <hdrhistogram::Histogram<u8>>::new_from::<u16>(&<hdrhistogram::Histogram<u16>>::new_from::<u32>(&<hdrhistogram::Histogram<u32>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_with_max(u64::MAX, 2u8).unwrap())));
    let sync_histogram = histogram.clone().into_sync();
    let mut recorder = sync_histogram.recorder();
    let _ = recorder.record_n_correct(4380101545244474084u64, 26u8, 7u64);
}
