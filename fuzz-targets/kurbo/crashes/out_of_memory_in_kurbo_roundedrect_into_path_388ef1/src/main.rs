use kurbo::RoundedRect;
use kurbo::RoundedRectRadii;

fn main() {
    let _ = <kurbo::RoundedRect as kurbo::Shape>::into_path(RoundedRect::new(1.0f64, 1.0f64, 0.0f64, 0.0f64, RoundedRectRadii::new(0.0f64, 0.0f64, 1.0f64, 1.0f64)), 0.0f64);
}
