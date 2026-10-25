use tiff::decoder::DecodingSampleBuffer;

fn main() {
    let mut decoding_sample_buffer = DecodingSampleBuffer::U16(vec![]);
    let decoding_sample_slice = decoding_sample_buffer.as_buffer(1usize);
}
