use av_data::pixel::Chromaton;
use av_data::pixel::ColorModel;
use av_data::pixel::Formaton;
use av_data::pixel::TrichromaticEncodingSystem;

fn main() {
    let _ = Formaton::new(ColorModel::Trichromatic(TrichromaticEncodingSystem::RGB), &[Chromaton::new(0u8, 0u8, false, 0u8, 0u8, 0u8, 0u8), Chromaton::packrgb(0u8, 0u8, 0u8, 0u8), Chromaton::packrgb(0u8, 0u8, 0u8, 0u8), Chromaton { h_ss: 0u8, v_ss: 0u8, packed: false, depth: 0u8, shift: 0u8, comp_offs: 0u8, next_elem: 0u8 }, Chromaton::yuvhb(0u8, 0u8, 0u8, 0u8), Chromaton::yuv8(0u8, 0u8, 0u8)], 0u8, false, false, false);
}
