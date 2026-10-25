use av_data::audiosample::ChannelMap;
use av_data::audiosample::Soniton;
use av_data::frame::AudioInfo;
use av_data::frame::MediaKind;

fn main() {
    let media_kind = MediaKind::Audio(AudioInfo { samples: 0usize, sample_rate: 0usize, map: ChannelMap::new(), format: std::sync::Arc::new(Soniton { bits: 0u8, be: false, packed: false, planar: false, float: false, signed: false }), block_len: None });
    let audio_info = media_kind.get_audio_info().unwrap();
    let _ = audio_info.size(0usize);
}
