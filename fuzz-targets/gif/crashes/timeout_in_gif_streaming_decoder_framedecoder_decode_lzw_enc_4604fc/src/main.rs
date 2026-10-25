use gif::DecodeOptions;
use gif::Frame;
use gif::streaming_decoder::FrameDecoder;

fn main() {
    let decode_options = DecodeOptions::new();
    let mut frame = Frame::from_indexed_pixels(0u16, 0u16, vec![], None);
    frame.make_lzw_pre_encoded();
    let mut frame_decoder = FrameDecoder::new(decode_options.clone());
    let _ = frame_decoder.decode_lzw_encoded_frame(&mut frame);
}
