use rav1e::bench::ec::WriterRecorder;

fn main() {
    let mut writer_base = WriterRecorder::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterRecorder> as rav1e::bench::ec::Writer>::write_quniform(&mut writer_base, 10000u32, 0u32);
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterRecorder> as rav1e::bench::ec::Writer>::symbol_bits(&writer_base, 1u32, &[u16::MAX, 0u16]);
}
