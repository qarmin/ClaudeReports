use rav1e::bench::ec::WriterEncoder;

fn main() {
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterEncoder> as rav1e::bench::ec::Writer>::count_signed_subexp_with_ref(&WriterEncoder::new(), 0i32, 0i32, 0i32, 100u8, 0i32);
}
