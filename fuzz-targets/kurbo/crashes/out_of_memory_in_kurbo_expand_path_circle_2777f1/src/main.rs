use kurbo::BezPath;
use kurbo::Diagonal2;
use kurbo::Join;

fn main() {
    let _ = kurbo::expand_path(<kurbo::BezPath as kurbo::Shape>::as_circle(&BezPath::with_capacity(1000000000usize)).unwrap(), Diagonal2::new(0.0f64, 0.0f64), Join::Round, 0.0f64, 0.0f64);
}
