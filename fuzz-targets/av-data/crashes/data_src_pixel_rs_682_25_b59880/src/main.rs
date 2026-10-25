use av_data::pixel::Chromaton;

fn main() {
    let chromaton = Chromaton::packrgb(85u8, 0u8, 0u8, 0u8);
    let _ = chromaton.to_string();
}
