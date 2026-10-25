use rav1e::bench::ec::WriterCounter;

fn main() {
    let mut writer_base = WriterCounter::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::Writer>::write_signed_subexp_with_ref(&mut writer_base, -1000000000i32, i32::MAX, 0i32, 0u8, 0i32);
}
