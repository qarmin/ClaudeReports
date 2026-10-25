use rav1e::bench::ec::WriterEncoder;

fn main() {
    let mut writer_base = WriterEncoder::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterEncoder> as rav1e::bench::ec::Writer>::write_quniform(&mut writer_base, 10000u32, 0u32);
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterEncoder> as rav1e::bench::ec::Writer>::symbol_bits(&writer_base, 1u32, &[u16::MAX, 10000u16, 0u16]);
}
