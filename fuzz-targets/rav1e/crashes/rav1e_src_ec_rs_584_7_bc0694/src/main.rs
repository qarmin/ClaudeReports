use rav1e::bench::ec::WriterEncoder;

fn main() {
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterEncoder> as rav1e::bench::ec::Writer>::symbol_bits(&WriterEncoder::new(), 0u32, &[u16::MAX, 0u16]);
}
