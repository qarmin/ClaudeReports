use kurbo::BezPath;
use kurbo::Rect;

fn main() {
    let _ = BezPath::from_vec(vec![<kurbo::Rect as kurbo::Shape>::to_path(&Rect::default(), 0.0f64).pop().unwrap()]);
}
