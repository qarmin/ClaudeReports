use kurbo::BezPath;
use kurbo::TranslateScale;
use kurbo::Vec2;

fn main() {
    let vec2 = <kurbo::Vec2 as ::core::ops::Neg>::neg(Vec2::default());
    let vec22 = vec2.round();
    let _ = <kurbo::TranslateScale as ::core::ops::Mul<kurbo::Circle>>::mul(TranslateScale::translate(vec22), <kurbo::BezPath as kurbo::Shape>::as_circle(&BezPath::with_capacity(1000000000usize)).unwrap());
}
