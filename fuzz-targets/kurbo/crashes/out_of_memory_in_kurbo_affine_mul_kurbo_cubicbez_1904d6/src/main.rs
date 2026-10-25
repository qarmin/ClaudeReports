use kurbo::Affine;
use kurbo::BezPath;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::Affine as ::core::ops::Mul<kurbo::CubicBez>>::mul(Affine::reflect(Vec2::from_angle(0.0f64).to_point(), Vec2::default()), BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap().to_cubic());
}
