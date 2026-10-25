use kurbo::Ellipse;
use kurbo::TranslateScale;

fn main() {
    let ellipse = Ellipse::default();
    let _ = <kurbo::TranslateScale as ::core::ops::Mul<kurbo::PathSeg>>::mul(TranslateScale::default(), <kurbo::Ellipse as kurbo::Shape>::to_path(&ellipse, 0.0f64).get_seg(0usize).unwrap());
}
