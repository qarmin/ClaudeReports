use kurbo::Affine;
use kurbo::Arc;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let join = Join::Miter;
    let affine = Affine::new([0.0f64, 1.0f64, 0.0f64, 1.0f64, 0.0f64, 0.0f64]);
    let f64_v = affine.frobenius_norm_squared();
    let _ = kurbo::expand_path_signed(Arc::new(Point::from((1.0f64, 0.0f64)), Vec2::new(f64::NEG_INFINITY, f64::INFINITY), 0.0f64, 1.0f64, -1e300f64), Diagonal2 { xx: -0.0f64, yy: -4.60134496568648e87f64 }.inv(), join, f64_v, 0.9999999999999999f64);
}
