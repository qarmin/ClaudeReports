use hdrhistogram::errors::AdditionError;
use hdrhistogram::serialization::DeserializeError;

fn main() {
    let _ = AdditionError::OtherAddendValueExceedsRange;
    let histogram = <hdrhistogram::Histogram<u32>>::new(0u8).unwrap();
    let sync_histogram = histogram.clone().into_sync();
    let mut recorder = sync_histogram.recorder();
    let _ = recorder.record_n_correct(u64::MAX, 1585395343u32, 1u64);
    let _ = DeserializeError::EncodedArrayTooLong;
}
