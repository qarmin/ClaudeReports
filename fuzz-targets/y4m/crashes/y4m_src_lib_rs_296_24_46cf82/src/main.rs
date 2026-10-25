use y4m::EncoderBuilder;
use y4m::Ratio;

fn main() {
    let encoder_builder = EncoderBuilder::new(usize::MAX, 12usize, Ratio::new(1usize, 0usize));
    let _ = encoder_builder.write_header::<Vec<u8>>(include_bytes!("mutated_0_Cargo.toml").to_vec());
}
