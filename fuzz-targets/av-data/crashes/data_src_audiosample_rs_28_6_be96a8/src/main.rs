use av_data::audiosample::ChannelMap;
use av_data::audiosample::formats::S32;
use av_data::frame::AudioInfo;

fn main() {
    let audio_info = AudioInfo::new(1usize, 0usize, ChannelMap::default(), std::sync::Arc::new(S32), Some(usize::MAX));
    let usize_v = audio_info.get_block_len().unwrap();
    let _ = audio_info.size(usize_v);
}
