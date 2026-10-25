use v_frame::chroma::ChromaSubsampling;
use v_frame::frame::FrameBuilder;

fn main() {
    let chroma_subsampling = ChromaSubsampling::Yuv444;
    let frame_builder = FrameBuilder::new(1000000000usize, 1usize, chroma_subsampling, 8u8);
    let frame = frame_builder.build::<u8>().unwrap();
    let _ = frame.plane(1usize);
}
