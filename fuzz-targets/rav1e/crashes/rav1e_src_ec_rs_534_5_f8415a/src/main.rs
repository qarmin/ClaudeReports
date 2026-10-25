use rav1e::bench::ec::WriterCounter;

fn main() {
    let mut writer_base = WriterCounter::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::Writer>::symbol::<4>(&mut writer_base, 0u32, &[u16::MAX, 0u16, 0u16, 1u16]);
}
