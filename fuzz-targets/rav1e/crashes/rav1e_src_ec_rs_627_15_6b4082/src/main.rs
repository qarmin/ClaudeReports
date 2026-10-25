use rav1e::bench::ec::WriterRecorder;

fn main() {
    let mut writer_base = WriterRecorder::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterRecorder> as rav1e::bench::ec::Writer>::write_quniform(&mut writer_base, u32::MAX, 0u32);
}
