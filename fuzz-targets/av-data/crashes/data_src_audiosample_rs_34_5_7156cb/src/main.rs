use av_data::audiosample::ChannelMap;
use av_data::audiosample::Soniton;
use av_data::frame::AudioInfo;
use av_data::frame::Frame;
use av_data::frame::MediaKind;

fn main() {
    let _ = Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::from(AudioInfo::new(usize::MAX, 0usize, ChannelMap::new(), std::sync::Arc::new(Soniton::new(1u8, false, true, false, false, false)), None)), None);
}
