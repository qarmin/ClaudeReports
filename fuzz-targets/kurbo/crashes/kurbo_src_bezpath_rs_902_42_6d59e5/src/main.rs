use kurbo::PathEl;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let path_el = PathEl::ClosePath;
    let _ = SimplifyBezPath::new(vec![path_el]);
}
