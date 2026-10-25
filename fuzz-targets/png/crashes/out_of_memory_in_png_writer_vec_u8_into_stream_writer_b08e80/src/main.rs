fn main() {
    let encoder = <png::Encoder<Vec<u8>>>::new(vec![], 1000000000u32, 1u32);
    let writer = encoder.write_header().unwrap();
    let stream_writer = writer.into_stream_writer().unwrap();
}
