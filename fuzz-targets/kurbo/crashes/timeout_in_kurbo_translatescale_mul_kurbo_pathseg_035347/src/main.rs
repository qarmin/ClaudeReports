use kurbo::Point;
use kurbo::RoundedRect;
use kurbo::RoundedRectRadii;
use kurbo::TranslateScale;

fn main() {
    let _ = <kurbo::TranslateScale as ::core::ops::Mul<kurbo::PathSeg>>::mul(TranslateScale::default(), <kurbo::RoundedRect as kurbo::Shape>::into_path(RoundedRect::from_points(Point { x: 0.5f64, y: 9007199254740992.0f64 }, Point::new(3.494079430643412e-158f64, f64::NEG_INFINITY), RoundedRectRadii::from_single_radius(2e-323f64)), 0.0f64).get_seg(256usize).unwrap());
}
