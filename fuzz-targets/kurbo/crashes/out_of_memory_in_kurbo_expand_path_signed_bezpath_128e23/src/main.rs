use kurbo::Diagonal2;
use kurbo::Ellipse;
use kurbo::Join;

fn main() {
    let _ = kurbo::expand_path_signed(<kurbo::Ellipse as kurbo::Shape>::into_path(Ellipse::default(), 0.0f64), Diagonal2 { xx: 0.0f64, yy: 0.0f64 }, Join::Round, 0.0f64, 1.0f64);
}
