use rav1e::bench::ec::WriterCounter;

fn main() {
    let mut writer_base = WriterCounter::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::Writer>::write_unsigned_subexp_with_ref(&mut writer_base, u32::MAX, 2258794559u32, 0u8, 2122739179u32);
}
