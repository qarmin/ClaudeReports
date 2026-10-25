use gif::Frame;

fn main() {
    let vec = vec![];
    let mut frame = Frame::from_grayscale_with_alpha(0u16, 0u16, &vec);
    frame.make_lzw_pre_encoded();
    let mut encoder = <gif::Encoder<Vec<u8>>>::new(vec![], 0u16, 0u16, &vec![]).unwrap();
    let _ = encoder.write_frame(&frame);
}
