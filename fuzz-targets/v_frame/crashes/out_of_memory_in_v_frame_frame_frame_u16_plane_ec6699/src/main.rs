use v_frame::chroma::ChromaSubsampling;
use v_frame::frame::FrameBuilder;

fn main() {
    let frame = FrameBuilder::new(10usize, 1000000000usize, ChromaSubsampling::Yuv422, 10u8).build::<u16>().unwrap();
    let _ = frame.plane(1usize);
}
