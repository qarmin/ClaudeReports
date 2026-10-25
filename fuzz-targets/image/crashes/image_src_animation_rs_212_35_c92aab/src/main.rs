use image::Delay;

fn main() {
    let _ = Delay::from_saturating_duration(std::time::Duration::new(1000000000u64, 1073741824u32 % 1_000_000_000));
}
