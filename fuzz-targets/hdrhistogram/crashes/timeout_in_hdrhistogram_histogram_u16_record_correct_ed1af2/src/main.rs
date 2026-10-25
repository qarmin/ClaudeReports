use hdrhistogram::serialization::V2DeflateSerializer;
use hdrhistogram::serialization::interval_log::IntervalLogWriterBuilder;

fn main() {
    let histogram = <hdrhistogram::Histogram<u64>>::new_from::<u32>(&<hdrhistogram::Histogram<u32>>::new_from::<u16>(&<hdrhistogram::Histogram<u16>>::new(0u8).unwrap()));
    let _ = IntervalLogWriterBuilder::default();
    let _ = V2DeflateSerializer::default();
    let mut histogram2 = <hdrhistogram::Histogram<u16>>::new_from::<u64>(&histogram);
    let _ = histogram2.record_correct(100000000000000000u64, 1u64);
}
