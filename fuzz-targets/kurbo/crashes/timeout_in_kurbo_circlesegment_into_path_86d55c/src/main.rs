use kurbo::Arc;
use kurbo::Cap;
use kurbo::CircleSegment;
use kurbo::DEFAULT_ACCURACY;
use kurbo::Diagonal2;
use kurbo::LineIntersection;
use kurbo::Moments;
use kurbo::PathSeg;
use kurbo::Point;
use kurbo::StrokeCtx;
use kurbo::SvgArc;
use kurbo::SvgParseError;
use kurbo::Vec2;
use kurbo::common::GAUSS_LEGENDRE_COEFFS_8_HALF;
use kurbo::simplify::SimplifyOptLevel;

fn main() {
    let svg_parse_error = SvgParseError::Wrong;
    let bez_path = <kurbo::CircleSegment as kurbo::Shape>::into_path(CircleSegment::new(<kurbo::Diagonal2 as ::core::ops::Mul<kurbo::Point>>::mul(Diagonal2::new(f64::INFINITY, 1.0f64), Point { x: 0.241f64, y: 0.0f64 }), 9007199254740992.0f64, 0.836f64, 0.5f64, -1.4174536431560663e60f64), 10000000000.0f64);
    let _ = <kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(Moments { moment_x: -1e300f64, moment_y: 1.0f64, moment_xx: 0.5f64, moment_xy: 1.0f64, moment_yy: f64::MAX }, Moments::default());
    let f64_v = DEFAULT_ACCURACY;
    let _ = LineIntersection { line_t: -0.0f64, segment_t: 308.0f64 };
    let diagonal2 = Diagonal2 { xx: 0.308f64, yy: 0.182f64 };
    let diagonal22 = diagonal2.abs();
    let vec2 = diagonal22.scale_normal(Vec2::from_angle(f64_v));
    let point = Point::from((8.3f64, 1e-310f64));
    let stroke_ctx = StrokeCtx::default();
    let _ = stroke_ctx.output();
    let _ = Cap::Round;
    let _ = SimplifyOptLevel::Subdivide;
    let _ = <kurbo::PathSeg as kurbo::ParamCurveNearest>::nearest(&PathSeg::from(<kurbo::BezPath as kurbo::Shape>::as_line(&bez_path).unwrap()), point, -1.0f64);
    let f64_v2 = vec2.length();
    let _ = GAUSS_LEGENDRE_COEFFS_8_HALF.to_vec();
    let _ = <kurbo::Circle as kurbo::Shape>::as_circle(&<kurbo::Rect as kurbo::Shape>::as_circle(&<kurbo::Arc as kurbo::Shape>::as_rect(&Arc::from_svg_arc(&SvgArc { from: point, to: point, radii: vec2, x_rotation: f64_v2, large_arc: false, sweep: true }).unwrap()).unwrap()).unwrap());
    let _ = svg_parse_error.to_string();
    let _ = SvgParseError::UninitializedPath;
}
