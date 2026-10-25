use av_data::audiosample::ChannelMap;
use av_data::audiosample::Soniton;
use av_data::frame::AudioInfo;
use av_data::frame::Frame;
use av_data::frame::MediaKind;

fn main() {
    <av_data::frame::Frame as av_data::frame::FrameBufferCopy>::copy_plane_to_buffer(&Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::from(AudioInfo::new(0usize, 0usize, ChannelMap::default_map(1usize), std::sync::Arc::new(Soniton { bits: 1u8, be: false, packed: false, planar: false, float: false, signed: false }), None)), None), 1usize, &mut vec![], 0usize);
}
