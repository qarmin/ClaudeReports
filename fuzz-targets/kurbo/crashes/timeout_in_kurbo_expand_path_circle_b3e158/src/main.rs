use kurbo::Circle;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;

fn main() {
    let point = Point::default();
    let _ = kurbo::expand_path(Circle { center: point, radius: 1.0f64 }, Diagonal2::new(0.0f64, 0.0f64), Join::Bevel, 0.0f64, 0.0f64);
}
