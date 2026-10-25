use av_data::audiosample::ChannelMap;
use av_data::audiosample::formats::U8;
use av_data::frame::AudioInfo;
use av_data::frame::Frame;
use av_data::frame::MediaKind;

fn main() {
    let _ = Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::from(AudioInfo::new(10000000000000000000usize, 1usize, ChannelMap::default_map(2usize), std::sync::Arc::new(U8), None)), None);
}
