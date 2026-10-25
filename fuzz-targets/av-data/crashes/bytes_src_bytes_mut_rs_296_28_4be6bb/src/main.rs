use av_data::audiosample::ChannelMap;
use av_data::audiosample::ChannelType;
use av_data::audiosample::formats::S16;
use av_data::frame::AudioInfo;
use av_data::frame::Frame;
use av_data::frame::MediaKind;

fn main() {
    let mut channel_map = ChannelMap::new();
    channel_map.add_channel(ChannelType::Cs);
    let _ = Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::from(AudioInfo { samples: 8341552203014512160usize, sample_rate: 0usize, map: channel_map.clone(), format: std::sync::Arc::new(S16), block_len: None }), None);
}
