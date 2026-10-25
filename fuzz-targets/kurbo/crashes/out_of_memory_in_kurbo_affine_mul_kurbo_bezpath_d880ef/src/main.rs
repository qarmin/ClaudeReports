use kurbo::Affine;
use kurbo::BezPath;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::Affine as ::core::ops::Mul<kurbo::BezPath>>::mul(Affine::translate::<kurbo::Vec2>(Vec2::ZERO), BezPath::with_capacity(1000000000000000usize));
}
