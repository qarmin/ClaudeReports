use gif::DecodeOptions;
use gif::Frame;
use gif::streaming_decoder::FrameDecoder;

fn main() {
    let vec = vec![];
    let mut frame = Frame::from_grayscale_with_alpha(0u16, 0u16, &vec);
    frame.make_lzw_pre_encoded();
    let mut frame_decoder = FrameDecoder::new(DecodeOptions::new());
    let _ = frame_decoder.decode_lzw_encoded_frame_into_buffer(&frame, &mut vec![]);
}
