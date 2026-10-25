use kurbo::Affine;
use kurbo::CircleSegment;
use kurbo::Ellipse;
use kurbo::RoundedRectRadii;
use kurbo::Size;

fn main() {
    let affine = Affine::scale(1.0f64);
    let affine2 = affine.pre_scale(0.0f64);
    let f64_v = affine2.frobenius_norm_squared();
    let circle_segment = CircleSegment { center: Size::default().to_rounded_rect(RoundedRectRadii::from_single_radius(0.0f64)).origin(), outer_radius: 0.0f64, inner_radius: 0.0f64, start_angle: 0.0f64, sweep_angle: f64_v };
    let ellipse = Ellipse::from_affine(affine);
    let _ = <kurbo::CircleSegment as kurbo::Shape>::winding(&circle_segment, <kurbo::PathSeg as kurbo::ParamCurve>::end(&<kurbo::Ellipse as kurbo::Shape>::into_path(ellipse, 0.0f64).get_seg(0usize).unwrap()));
}
