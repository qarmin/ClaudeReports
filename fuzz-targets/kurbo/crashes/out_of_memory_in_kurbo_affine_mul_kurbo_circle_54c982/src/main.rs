use kurbo::Affine;
use kurbo::BezPath;

fn main() {
    let _ = <kurbo::Affine as ::core::ops::Mul<kurbo::Circle>>::mul(Affine::new([0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64]), <kurbo::BezPath as kurbo::Shape>::as_circle(&BezPath::with_capacity(1000000000usize)).unwrap());
}
