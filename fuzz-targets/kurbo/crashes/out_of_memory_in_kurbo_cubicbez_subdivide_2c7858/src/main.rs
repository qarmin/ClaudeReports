use kurbo::Axis;
use kurbo::BezPath;
use kurbo::Circle;
use kurbo::CubicBez;
use kurbo::Point;

fn main() {
    let f64_v = <kurbo::Circle as kurbo::Shape>::area(&Circle::new(Point::from((0.0f32, 0.0f32)), 0.0f64));
    let axis = Axis::Horizontal;
    let axis2 = axis.cross();
    let axis3 = axis2.cross();
    let point = axis3.pack_point(f64_v, 0.0f64);
    let _ = <kurbo::CubicBez as kurbo::ParamCurve>::subdivide(&CubicBez { p0: point, p1: <kurbo::BezPath as kurbo::Shape>::as_line(&BezPath::with_capacity(1000000000usize)).unwrap().midpoint(), p2: point, p3: point });
}
