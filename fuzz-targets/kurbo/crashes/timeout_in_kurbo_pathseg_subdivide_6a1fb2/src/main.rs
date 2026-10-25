use kurbo::Axis;
use kurbo::LineIntersection;
use kurbo::PathSeg;
use kurbo::RoundedRect;
use kurbo::RoundedRectRadii;
use kurbo::Size;
use kurbo::Triangle;
use kurbo::Vec2;

fn main() {
    let vec2 = Vec2::ZERO;
    let f64_v = vec2.get_coord(Axis::Horizontal);
    let _ = Size::default();
    let _ = LineIntersection { line_t: -1.5520280845898103e-89f64, segment_t: 0.9999999999999999f64 };
    let _ = Triangle::from_coords((423.0f64, -0.0f64), (f64::INFINITY, 890.0f64), (0.625f64, -50.5f64));
    let axis = Axis::Vertical;
    let _ = axis.pack_size(-14.7f64, f64::NAN);
    let rounded_rect = RoundedRect::new(4.289189471386518e-131f64, 0.285f64, -0.0f64, 2.2250738585072014e-308f64, RoundedRectRadii { top_left: -8.797486174408505e-240f64, top_right: 102.0f64, bottom_right: 925.0f64, bottom_left: f64_v });
    let _ = <kurbo::PathSeg as kurbo::ParamCurve>::subdivide(&PathSeg::from(<kurbo::BezPath as kurbo::Shape>::as_line(&<kurbo::RoundedRect as kurbo::Shape>::to_path(&rounded_rect, 1e-310f64)).unwrap()));
}
