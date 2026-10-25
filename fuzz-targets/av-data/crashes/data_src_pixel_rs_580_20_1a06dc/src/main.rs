use av_data::pixel::Chromaton;

fn main() {
    let chromaton = Chromaton::pal8(0u8);
    let _ = chromaton.get_linesize(1usize, 0usize);
}
