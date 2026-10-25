use kurbo::BezPath;
use kurbo::Diagonal2;
use kurbo::Join;

fn main() {
    let _ = kurbo::expand_path(BezPath::with_capacity(1000000000usize), Diagonal2::new(0.0f64, 0.0f64), Join::Miter, 1.0f64, 1.0f64);
}
