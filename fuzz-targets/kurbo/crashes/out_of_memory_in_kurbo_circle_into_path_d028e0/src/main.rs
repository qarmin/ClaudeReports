use kurbo::Triangle;

fn main() {
    let _ = <kurbo::Circle as kurbo::Shape>::into_path(Triangle::from_coords((1.0f64, 0.0f64), (1.0f64, 1.0f64), (0.0f64, 0.0f64)).circumscribed_circle(), 0.0f64);
}
