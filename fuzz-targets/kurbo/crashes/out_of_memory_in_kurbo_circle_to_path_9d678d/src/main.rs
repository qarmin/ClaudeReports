use kurbo::Circle;
use kurbo::Insets;
use kurbo::Point;

fn main() {
    let insets = <kurbo::Insets as ::core::ops::Mul<f64>>::mul(Insets::uniform_xy(0.0f64, 0.0f64), 0.0f64);
    let f64_v = insets.x_value();
    let _ = <kurbo::Circle as kurbo::Shape>::to_path(&Circle { center: Point { x: 1.0f64, y: 0.0f64 }, radius: 1.0f64 }, f64_v);
}
