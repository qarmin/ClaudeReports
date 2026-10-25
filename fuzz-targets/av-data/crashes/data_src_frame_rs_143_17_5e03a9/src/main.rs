use av_data::frame::Frame;
use av_data::frame::FrameType;
use av_data::frame::MediaKind;
use av_data::frame::VideoInfo;
use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;

fn main() {
    let media_kind = MediaKind::from(VideoInfo { width: 1usize, height: 531707584965882534usize, flipped: false, frame_type: FrameType::P, format: std::sync::Arc::new(Formaton::new(ColorModel::CMYK, &[Chromaton::pal8(0u8), Chromaton::packrgb(10u8, 0u8, 0u8, 0u8)], 0u8, false, false, false)), bits: 0u8 });
    let _ = Frame::new_default_frame::<av_data::frame::MediaKind>(media_kind.clone(), None);
}
