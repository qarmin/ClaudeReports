use av_data::frame::Frame;
use av_data::frame::FrameType;
use av_data::frame::MediaKind;
use av_data::frame::VideoInfo;
use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;
use av_data::pixel::TrichromaticEncodingSystem;

fn main() {
    let trichromatic_encoding_system = TrichromaticEncodingSystem::RGB;
    let media_kind = MediaKind::Video(VideoInfo { width: 0usize, height: 0usize, flipped: false, frame_type: FrameType::B, format: std::sync::Arc::new(Formaton::new(ColorModel::Trichromatic(trichromatic_encoding_system), &[Chromaton { h_ss: 0u8, v_ss: 100u8, packed: false, depth: 0u8, shift: 0u8, comp_offs: 0u8, next_elem: 0u8 }], 0u8, false, false, false)), bits: 0u8 });
    let media_kind2 = MediaKind::from(media_kind.get_video_info().unwrap());
    let _ = Frame::new_default_frame::<av_data::frame::MediaKind>(MediaKind::Video(media_kind2.get_video_info().unwrap()), None);
}
