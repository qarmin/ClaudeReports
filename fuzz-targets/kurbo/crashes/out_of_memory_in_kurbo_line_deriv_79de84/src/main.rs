use kurbo::CircleSegment;
use kurbo::Join;
use kurbo::MAX_EXTREMA;
use kurbo::Moments;
use kurbo::Point;

fn main() {
    let _ = kurbo::cubics_to_quadratic_splines(&[], 0.805f64);
    let _ = Moments { moment_x: 1e300f64, moment_y: 0.631f64, moment_xx: 1e300f64, moment_xy: 243.0f64, moment_yy: f64::MAX };
    let _ = Point::ORIGIN;
    let _ = MAX_EXTREMA;
    let _ = <kurbo::Line as kurbo::ParamCurveDeriv>::deriv(&<kurbo::BezPath as kurbo::Shape>::as_line(&<kurbo::CircleSegment as kurbo::Shape>::to_path(&CircleSegment { center: Point::ZERO, outer_radius: 876.0f64, inner_radius: f64::NEG_INFINITY, start_angle: 2.2250738585072014e-308f64, sweep_angle: 0.047f64 }, f64::MIN)).unwrap());
    let _ = Join::Round;
}
