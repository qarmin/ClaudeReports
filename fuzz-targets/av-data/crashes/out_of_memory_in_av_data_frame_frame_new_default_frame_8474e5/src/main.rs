use av_data::audiosample::ChannelMap;
use av_data::audiosample::ChannelType;
use av_data::audiosample::formats::S16;
use av_data::frame::AudioInfo;
use av_data::frame::Frame;
use av_data::frame::MediaKind;

fn main() {
    let mut channel_map = ChannelMap::default();
    channel_map.add_channels(&[ChannelType::Cs]);
    let _ = Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::Audio(AudioInfo::new(4294967296usize, 0usize, channel_map.clone(), std::sync::Arc::new(S16), None)), None);
}
