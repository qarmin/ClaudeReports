use rav1e::bench::ec::WriterCounter;

fn main() {
    let mut writer_base = WriterCounter::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::StorageBackend>::store(&mut writer_base, 1u16, 1000u16, 1u16);
}
