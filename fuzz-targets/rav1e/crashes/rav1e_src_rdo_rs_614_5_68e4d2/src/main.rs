use rav1e::bench::rdo::Distortion;
use rav1e::bench::rdo::DistortionScale;

fn main() {
    let _ = <rav1e::bench::rdo::Distortion as ::core::ops::Mul<rav1e::bench::rdo::DistortionScale>>::mul(Distortion(u64::MAX), DistortionScale::inv_mean(&[DistortionScale(945063371u32)]));
}
