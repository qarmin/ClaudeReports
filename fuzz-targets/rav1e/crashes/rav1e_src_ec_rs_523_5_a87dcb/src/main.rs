use rav1e::bench::ec::WriterRecorder;

fn main() {
    let mut writer_base = WriterRecorder::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterRecorder> as rav1e::bench::ec::Writer>::symbol::<1>(&mut writer_base, 1u32, &[0u16]);
}
