use png::UnfilterRegion;

fn main() {
    let _ = UnfilterRegion::default();
    let encoder = <png::Encoder<Vec<u8>>>::new(vec![], 1u32, 1u32);
    let writer = encoder.write_header().unwrap();
    let stream_writer = writer.into_stream_writer_with_size(usize::MAX).unwrap();
}
