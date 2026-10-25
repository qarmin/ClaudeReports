use av_data::frame::Frame;
use av_data::frame::FrameType;
use av_data::frame::MediaKind;
use av_data::frame::VideoInfo;
use av_data::pixel::ChromaLocation;
use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::ColorPrimaries;
use av_data::pixel::Formaton;
use av_data::pixel::MatrixCoefficients;
use av_data::pixel::TransferCharacteristic;

fn main() {
    let video_info = VideoInfo { width: 0usize, height: 0usize, flipped: false, frame_type: FrameType::OTHER, format: std::sync::Arc::new(Formaton { model: ColorModel::LAB, primaries: ColorPrimaries::BT2020, xfer: TransferCharacteristic::BT1361E, matrix: MatrixCoefficients::BT709, chroma_location: ChromaLocation::Center, components: 0u8, comp_info: [None, None, None, None, Some(Chromaton::pal8(1u8))], elem_size: 0u8, be: false, alpha: false, palette: false }), bits: 0u8 };
    <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer(&Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::Video(video_info.clone()), None), 0usize, &mut vec![], 1usize);
}
