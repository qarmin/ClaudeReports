use rav1e::bench::rdo::DistortionScale;

fn main() {
    let u64_v = rav1e::version::major();
    let _ = DistortionScale::new(0u64, u64_v);
}
