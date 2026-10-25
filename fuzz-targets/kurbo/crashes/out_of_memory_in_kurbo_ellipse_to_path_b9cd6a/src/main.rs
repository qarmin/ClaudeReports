use kurbo::Affine;
use kurbo::Ellipse;

fn main() {
    let _ = <kurbo::Ellipse as kurbo::Shape>::to_path(&Ellipse::from_affine(Affine::FLIP_Y), -0.0f64);
}
