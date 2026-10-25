use v_frame::chroma::ChromaSubsampling;
use v_frame::frame::FrameBuilder;

fn main() {
    let frame_builder = FrameBuilder::new(100usize, 10000000000000000000usize, ChromaSubsampling::Yuv444, 16u8);
    let _ = frame_builder.build::<u16>();
}
