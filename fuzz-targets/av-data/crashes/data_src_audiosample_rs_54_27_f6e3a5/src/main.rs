use av_data::audiosample::ChannelMap;
use av_data::audiosample::Soniton;
use av_data::frame::AudioInfo;

fn main() {
    let audio_info = AudioInfo { samples: usize::MAX, sample_rate: 6usize, map: ChannelMap::default(), format: std::sync::Arc::new(AudioInfo::new(2usize, 14208578711885967420usize, ChannelMap::new(), std::sync::Arc::new(Soniton { bits: 255u8, be: false, packed: true, planar: true, float: true, signed: false }), Some(0usize)).get_format()), block_len: Some(1usize) };
    let _ = audio_info.size(4294967295usize);
}
