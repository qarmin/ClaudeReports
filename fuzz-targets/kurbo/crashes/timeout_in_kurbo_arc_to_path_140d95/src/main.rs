use kurbo::Arc;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::Arc as kurbo::Shape>::to_path(&Arc::new(Point::default(), Vec2 { x: 0.0f64, y: 65536.0f64 }, 0.0f64, -1e300f64, 1e-310f64), f64::NAN);
}
