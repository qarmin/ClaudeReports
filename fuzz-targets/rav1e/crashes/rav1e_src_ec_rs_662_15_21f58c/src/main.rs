use rav1e::bench::ec::WriterEncoder;

fn main() {
    let mut writer_base = WriterEncoder::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterEncoder> as rav1e::bench::ec::Writer>::write_unsigned_subexp_with_ref(&mut writer_base, u32::MAX, u32::MAX, 0u8, 0u32);
}
