use av_data::audiosample::ChannelMap;
use av_data::frame::Frame;
use av_data::frame::FrameType;
use av_data::frame::MediaKind;
use av_data::frame::VideoInfo;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;

fn main() {
    let channel_map = ChannelMap::new();
    let usize_v = channel_map.len();
    let color_model = ColorModel::LAB;
    <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer(&Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::from(VideoInfo::new(0usize, 0usize, false, FrameType::OTHER, std::sync::Arc::new(Formaton::new(color_model, &[], 0u8, false, false, false)))), None), 0usize, &mut vec![], usize_v);
}
