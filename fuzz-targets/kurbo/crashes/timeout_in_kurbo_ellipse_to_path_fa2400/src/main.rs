use kurbo::Affine;
use kurbo::Cap;
use kurbo::DEFAULT_ACCURACY;
use kurbo::Diagonal2;
use kurbo::Ellipse;
use kurbo::Join;
use kurbo::RoundedRectRadii;
use kurbo::Stroke;
use kurbo::StrokeCtx;
use kurbo::SvgParseError;
use kurbo::Vec2;

fn main() {
    let _ = Stroke::default();
    let _ = Cap::Square;
    let f64_v = DEFAULT_ACCURACY;
    let _ = <kurbo::Ellipse as kurbo::Shape>::to_path(&<kurbo::Ellipse as ::core::ops::Add<kurbo::Vec2>>::add(Ellipse::from_affine(<kurbo::Affine as ::core::ops::Mul<kurbo::Affine>>::mul(Affine::FLIP_X, Affine::rotate(-4.34e22f64))), Vec2::default()), 0.0f64);
    let _ = StrokeCtx::default();
    let _ = Diagonal2 { xx: f64::NAN, yy: 922.0f64 };
    let _ = Join::Bevel;
    let svg_parse_error = SvgParseError::Wrong;
    let _ = svg_parse_error.to_string();
    let _ = RoundedRectRadii::new(-0.0f64, f64_v, 1e-310f64, 1.0f64);
}
