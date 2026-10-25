use rav1e::bench::ec::WriterEncoder;

fn main() {
    let _ = <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterEncoder> as rav1e::bench::ec::Writer>::symbol_bits(&WriterEncoder::new(), 1u32, &[0u16, 10000u16, 0u16]);
}
