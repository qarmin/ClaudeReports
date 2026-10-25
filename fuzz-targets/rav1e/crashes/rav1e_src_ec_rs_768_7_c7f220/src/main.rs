use rav1e::bench::ec::WriterCounter;

fn main() {
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::Writer>::count_signed_subexp_with_ref(&WriterCounter::new(), 0i32, i32::MIN, 0i32, 0u8, 0i32);
}
