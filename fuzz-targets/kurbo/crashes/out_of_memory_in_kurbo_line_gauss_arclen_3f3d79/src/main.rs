use kurbo::CircleSegment;
use kurbo::Point;

fn main() {
    let point = Point::from(Point::ORIGIN);
    let _ = <kurbo::Line as kurbo::ParamCurveDeriv>::gauss_arclen(&<kurbo::BezPath as kurbo::Shape>::as_line(&<kurbo::CircleSegment as kurbo::Shape>::to_path(&CircleSegment::new(point, 0.0f64, f64::NEG_INFINITY, f64::INFINITY, 1.0f64), 0.0f64)).unwrap(), &[]);
}
