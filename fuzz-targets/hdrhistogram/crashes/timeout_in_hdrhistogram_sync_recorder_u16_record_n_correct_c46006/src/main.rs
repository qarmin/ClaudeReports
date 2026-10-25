fn main() {
    let sync_histogram = <hdrhistogram::SyncHistogram<u16> as ::core::convert::From<hdrhistogram::Histogram<u16>>>::from(<hdrhistogram::Histogram<u16>>::new(0u8).unwrap());
    let mut recorder = sync_histogram.recorder();
    let _ = recorder.record_n_correct(10000000000000000000u64, 82u16, 1000u64);
}
