use rav1e::bench::ec::WriterCounter;

fn main() {
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::Writer>::count_unsigned_subexp_with_ref(&WriterCounter::new(), 1u32, u32::MAX, 1u8, 1000000000u32);
}
