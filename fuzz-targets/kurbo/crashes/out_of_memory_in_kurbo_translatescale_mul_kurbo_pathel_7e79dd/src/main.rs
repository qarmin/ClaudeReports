use kurbo::BezPath;
use kurbo::TranslateScale;

fn main() {
    let _ = <kurbo::TranslateScale as ::core::ops::Mul<kurbo::PathEl>>::mul(TranslateScale::scale(0.0f64), BezPath::with_capacity(10000000000000000usize).pop().unwrap());
}
