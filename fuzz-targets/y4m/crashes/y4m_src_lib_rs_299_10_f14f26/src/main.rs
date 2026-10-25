use y4m::Ratio;

fn main() {
    let encoder_builder = y4m::encode(usize::MAX, 0usize, Ratio { num: 0usize, den: 0usize });
    let _ = encoder_builder.write_header::<Vec<u8>>(vec![]);
}
