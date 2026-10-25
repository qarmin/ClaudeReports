use kurbo::Affine;
use kurbo::Ellipse;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::Circle as ::core::ops::Add<kurbo::Vec2>>::add(<kurbo::BezPath as kurbo::Shape>::as_circle(&<kurbo::Ellipse as kurbo::Shape>::to_path(&Ellipse::from_affine(Affine::scale_non_uniform(1.0f64, 0.0f64)), 0.0f64)).unwrap(), Vec2 { x: 0.0f64, y: 0.0f64 });
}
