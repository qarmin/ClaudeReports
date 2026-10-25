use rav1e::bench::ec::WriterCounter;

fn main() {
    let writer_base = WriterCounter::new();
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::Writer>::count_signed_subexp_with_ref(&writer_base, 0i32, 1i32, 0i32, 0u8, i32::MIN);
}
