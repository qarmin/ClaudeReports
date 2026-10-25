use av_data::pixel::Chromaton;

fn main() {
    let chromaton = Chromaton::new(0u8, 1u8, false, 0u8, 0u8, 0u8, 0u8);
    let _ = chromaton.get_data_size(0usize, usize::MAX, 0usize);
}
