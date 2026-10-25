use kurbo::Affine;
use kurbo::Ellipse;

fn main() {
    let _ = <kurbo::PathSeg as kurbo::Shape>::path_elements(&<kurbo::Ellipse as kurbo::Shape>::to_path(&Ellipse::from_affine(Affine::rotate(0.0f64)), 0.0f64).get_seg(0usize).unwrap(), 0.0f64);
}
