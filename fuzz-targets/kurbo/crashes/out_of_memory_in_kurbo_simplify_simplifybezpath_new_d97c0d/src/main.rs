use kurbo::BezPath;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let _ = SimplifyBezPath::new(vec![BezPath::with_capacity(1000000000usize).pop().unwrap()]);
}
