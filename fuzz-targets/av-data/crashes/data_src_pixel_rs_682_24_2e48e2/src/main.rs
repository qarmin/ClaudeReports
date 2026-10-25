use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;

fn main() {
    let formaton = Formaton::new(ColorModel::CMYK, &[Chromaton::packrgb(0u8, 100u8, 0u8, 0u8)], 0u8, false, false, false);
    let _ = formaton.to_string();
}
