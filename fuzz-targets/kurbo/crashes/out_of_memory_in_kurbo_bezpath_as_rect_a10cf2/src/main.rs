use kurbo::Ellipse;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::as_rect(&<kurbo::Ellipse as kurbo::Shape>::into_path(Ellipse::default(), 0.0f64));
}
