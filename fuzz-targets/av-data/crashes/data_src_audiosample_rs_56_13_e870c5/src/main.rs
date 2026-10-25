use av_data::audiosample::ChannelMap;
use av_data::audiosample::formats::S16;
use av_data::frame::AudioInfo;

fn main() {
    let audio_info = AudioInfo { samples: usize::MAX, sample_rate: 7usize, map: ChannelMap::default(), format: std::sync::Arc::new(S16), block_len: None };
    let _ = audio_info.size(65536usize);
}
