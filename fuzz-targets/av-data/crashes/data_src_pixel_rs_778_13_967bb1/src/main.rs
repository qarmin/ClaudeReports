use av_data::frame::FrameType;
use av_data::frame::VideoInfo;
use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;

fn main() {
    let _ = VideoInfo::new(0usize, 0usize, false, FrameType::B, std::sync::Arc::new(Formaton::new(ColorModel::HSV, &[Chromaton::pal8(0u8), Chromaton::packrgb(255u8, 0u8, 0u8, 0u8)], 0u8, false, false, false)));
}
