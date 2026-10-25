use gif::streaming_decoder::StreamingDecoder;

fn main() {
    let streaming_decoder = StreamingDecoder::new();
    let _ = streaming_decoder.current_frame();
}
