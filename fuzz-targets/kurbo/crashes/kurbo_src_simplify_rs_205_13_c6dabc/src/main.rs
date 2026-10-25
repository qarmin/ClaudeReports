use kurbo::simplify::SimplifyBezPath;

fn main() {
    let _ = kurbo::fit_to_cubic(&SimplifyBezPath::new(vec![]), 0.0f64..0.0f64, 0.0f64);
}
