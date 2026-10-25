use rav1e::bench::ec::WriterCounter;

fn main() {
    let writer_base = WriterCounter::new();
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::Writer>::symbol_bits(&writer_base, 1u32, &[0u16]);
}
