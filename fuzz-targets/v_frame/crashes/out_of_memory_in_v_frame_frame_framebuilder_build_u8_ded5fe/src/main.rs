use v_frame::chroma::ChromaSubsampling;
use v_frame::frame::FrameBuilder;

fn main() {
    let frame_builder = FrameBuilder::new(1usize, 1000000000usize, ChromaSubsampling::Yuv444, 8u8);
    let _ = frame_builder.build::<u8>();
}
