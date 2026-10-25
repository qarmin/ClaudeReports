use kurbo::PathEl;
use kurbo::simplify::SimplifyOptions;

fn main() {
    let _ = kurbo::simplify::simplify_bezpath(vec![PathEl::ClosePath], 0.0f64, &SimplifyOptions::default());
}
