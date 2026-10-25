use v_frame::chroma::ChromaSubsampling;
use v_frame::frame::FrameBuilder;

fn main() {
    let frame_builder = FrameBuilder::new(4294967295usize, 4294967295usize, ChromaSubsampling::Monochrome, 10u8);
    let _ = frame_builder.build::<u16>();
}
