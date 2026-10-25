fn main() {
    let histogram = <hdrhistogram::Histogram<u64>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_from::<u8>(&<hdrhistogram::Histogram<u8>>::new_from::<u16>(&<hdrhistogram::Histogram<u16>>::new_with_max(u64::MAX, 1u8).unwrap())));
    let sync_histogram = histogram.clone().into_sync();
    let mut recorder = sync_histogram.recorder();
    let _ = recorder.record_correct(18422672277416534739u64, 4294967295u64);
}
