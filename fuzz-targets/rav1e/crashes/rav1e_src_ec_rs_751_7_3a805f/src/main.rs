use rav1e::bench::ec::WriterEncoder;

fn main() {
    let mut writer_base = WriterEncoder::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterEncoder> as rav1e::bench::ec::Writer>::write_signed_subexp_with_ref(&mut writer_base, 0i32, 1i32, i32::MIN, 0u8, 0i32);
}
