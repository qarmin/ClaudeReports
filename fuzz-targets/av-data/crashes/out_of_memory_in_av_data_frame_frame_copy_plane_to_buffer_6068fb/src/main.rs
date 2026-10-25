use av_data::audiosample::ChannelMap;
use av_data::audiosample::formats::S32;
use av_data::frame::AudioInfo;
use av_data::frame::Frame;
use av_data::frame::MediaKind;

fn main() {
    <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer(&Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::Audio(AudioInfo::new(1000000000usize, 1usize, ChannelMap::default_map(1usize), std::sync::Arc::new(S32), None)), None), 0usize, &mut vec![], 0usize);
}
