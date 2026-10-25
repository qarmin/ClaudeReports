use rav1e::bench::rdo::RawDistortion;

fn main() {
    let _ = <rav1e::bench::rdo::RawDistortion as ::core::ops::Mul<rav1e::bench::rdo::DistortionScale>>::mul(RawDistortion::new(10000000000000000000u64), rav1e::bench::rdo::distortion_scale_for(0.866f64, 0.9999999999999999f64));
}
