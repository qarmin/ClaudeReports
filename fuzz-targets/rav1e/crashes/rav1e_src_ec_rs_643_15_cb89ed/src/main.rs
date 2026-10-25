use rav1e::bench::ec::WriterEncoder;

fn main() {
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterEncoder> as rav1e::bench::ec::Writer>::count_quniform(&WriterEncoder::new(), u32::MAX, 0u32);
}
