use av_data::pixel::Chromaton;

fn main() {
    let chromaton = Chromaton::yuv8(100u8, 0u8, 0u8);
    let _ = chromaton.get_linesize(0usize, 0usize);
}
