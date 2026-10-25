use kurbo::Affine;
use kurbo::Axis;
use kurbo::LineIntersection;
use kurbo::MAX_EXTREMA;
use kurbo::Moments;
use kurbo::QuadSpline;
use kurbo::Rect;
use kurbo::RoundedRectRadii;
use kurbo::StrokeCtx;
use kurbo::TranslateScale;
use kurbo::Vec2;

fn main() {
    let axis = Axis::Vertical;
    let _ = axis.pack_size(255.0f64, f64::MAX);
    let _ = TranslateScale::translate(Vec2::ZERO);
    let stroke_ctx = StrokeCtx::default();
    let _ = stroke_ctx.output();
    let rounded_rect_radii = RoundedRectRadii { top_left: f64::INFINITY, top_right: f64::NEG_INFINITY, bottom_right: 0.9999999999999999f64, bottom_left: 0.283f64 };
    let _ = rounded_rect_radii.clamp(10000000000.0f64);
    let _ = <kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(<kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(Moments { moment_x: 0.478f64, moment_y: 0.573f64, moment_xx: -1e300f64, moment_xy: 557.0f64, moment_yy: f64::MAX }, Moments::default()), <kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(Moments::default(), <kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(Moments::default(), Moments::default())));
    let _ = LineIntersection { line_t: 0.266f64, segment_t: -69.6f64 };
    let stroke_ctx2 = StrokeCtx::default();
    let _ = stroke_ctx2.output();
    let affine = Affine::skew(f64::MIN, 71.8f64);
    let affine2 = affine.pre_skew(-1e300f64, -80.9f64);
    let _ = affine2.inverse();
    let quad_spline = QuadSpline::new(vec![]);
    let _ = quad_spline.points();
    let _ = <kurbo::RoundedRect as kurbo::Shape>::to_path(&Rect::new(-4.9786148735079455e270f64, 9007199254740992.0f64, 1e300f64, f64::MIN).inflate(22.5f64, 537.0f64).to_rounded_rect(rounded_rect_radii), 65.7f64);
    let _ = MAX_EXTREMA;
}
