use rav1e::bench::ec::WriterRecorder;

fn main() {
    let mut writer_base = WriterRecorder::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterRecorder> as rav1e::bench::ec::Writer>::symbol::<4>(&mut writer_base, 1u32, &[u16::MAX, 0u16, 0u16, 0u16]);
}
