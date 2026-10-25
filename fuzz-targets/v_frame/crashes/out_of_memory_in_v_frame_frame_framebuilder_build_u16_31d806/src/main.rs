use v_frame::chroma::ChromaSubsampling;
use v_frame::frame::FrameBuilder;

fn main() {
    let chroma_subsampling = ChromaSubsampling::Yuv422;
    let frame_builder = FrameBuilder::new(1000000000usize, 1usize, chroma_subsampling, 10u8);
    let frame_builder2 = frame_builder.luma_padding_top(0usize);
    let _ = frame_builder2.build::<u16>();
}
