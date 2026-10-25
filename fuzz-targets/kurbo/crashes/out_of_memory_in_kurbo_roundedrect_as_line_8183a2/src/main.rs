use kurbo::Affine;
use kurbo::Ellipse;

fn main() {
    let _ = <kurbo::RoundedRect as kurbo::Shape>::as_line(&<kurbo::BezPath as kurbo::Shape>::as_rounded_rect(&<kurbo::Ellipse as kurbo::Shape>::into_path(Ellipse::from_affine(Affine::IDENTITY), 0.0f64)).unwrap());
}
