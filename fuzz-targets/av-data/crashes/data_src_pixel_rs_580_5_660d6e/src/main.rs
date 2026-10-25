use av_data::pixel::Chromaton;

fn main() {
    let chromaton = Chromaton::yuvhb(0u8, 0u8, 1u8, 0u8);
    let _ = chromaton.get_data_size(0usize, 0usize, 0usize);
}
