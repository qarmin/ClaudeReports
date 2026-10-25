use rav1e::EncoderConfig;
use rav1e::prelude::Sequence;

fn main() {
    let sequence = Sequence::new(&EncoderConfig::with_speed_preset(0u8));
    let _ = sequence.get_relative_dist(2498367332u32, 1000000000u32);
}
