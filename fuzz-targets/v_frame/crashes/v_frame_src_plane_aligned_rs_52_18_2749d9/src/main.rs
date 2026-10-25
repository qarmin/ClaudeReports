use v_frame::chroma::ChromaSubsampling;
use v_frame::frame::FrameBuilder;

fn main() {
    let chroma_subsampling = ChromaSubsampling::Yuv420;
    let frame_builder = FrameBuilder::new(16usize, 1440161638835863816usize, chroma_subsampling, 10u8);
    let frame_builder2 = frame_builder.luma_padding_right(0usize);
    let _ = frame_builder2.build::<u16>();
}
