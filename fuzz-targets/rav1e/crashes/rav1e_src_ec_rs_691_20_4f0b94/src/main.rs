use rav1e::bench::ec::WriterCounter;

fn main() {
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::Writer>::count_unsigned_subexp_with_ref(&WriterCounter::new(), 0u32, 0u32, 31u8, 0u32);
}
