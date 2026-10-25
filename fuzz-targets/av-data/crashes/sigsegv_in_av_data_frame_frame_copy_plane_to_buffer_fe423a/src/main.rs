use av_data::frame::Frame;
use av_data::frame::FrameType;
use av_data::frame::MediaKind;
use av_data::frame::VideoInfo;
use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;

fn main() {
    let media_kind = MediaKind::Video(VideoInfo::new(1000000000usize, 1usize, false, FrameType::OTHER, std::sync::Arc::new(Formaton::new(ColorModel::CMYK, &[Chromaton::yuvhb(0u8, 0u8, 1u8, 0u8)], 0u8, false, false, false))));
    <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer(&Frame::new_default_frame::<av_data::frame::MediaKind>(media_kind.clone(), None), 0usize, &mut vec![66u8], 1usize);
}
