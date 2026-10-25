use rav1e::bench::ec::WriterCounter;

fn main() {
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::Writer>::symbol_bits(&WriterCounter::new(), 2u32, &[0u16, u16::MAX, 0u16]);
}
