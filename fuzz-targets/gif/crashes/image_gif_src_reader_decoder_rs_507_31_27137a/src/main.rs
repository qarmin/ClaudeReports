use gif::streaming_decoder::StreamingDecoder;

fn main() {
    let mut streaming_decoder = StreamingDecoder::new();
    let _ = streaming_decoder.current_frame_mut();
}
