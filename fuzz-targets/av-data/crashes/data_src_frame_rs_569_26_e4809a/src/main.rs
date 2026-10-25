use av_data::audiosample::ChannelMap;
use av_data::frame::Frame;
use av_data::frame::FrameType;
use av_data::frame::MediaKind;
use av_data::frame::VideoInfo;
use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;

fn main() {
    let chromaton = Chromaton::pal8(1u8);
    let channel_map = ChannelMap::new();
    let bool_v = channel_map.is_empty();
    let video_info = VideoInfo::from(VideoInfo { width: 0usize, height: 0usize, flipped: false, frame_type: FrameType::P, format: std::sync::Arc::new(Formaton::new(ColorModel::CMYK, &[chromaton], 0u8, false, false, bool_v)), bits: 0u8 });
    <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer(&Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::Video(video_info.clone()), None), 0usize, &mut vec![], 0usize);
}
