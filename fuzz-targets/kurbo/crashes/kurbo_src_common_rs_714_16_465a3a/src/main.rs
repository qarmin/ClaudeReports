fn main() {
    let _ = kurbo::common::solve_itp(|_| 1 as f64, 0.0f64, f64::NEG_INFINITY, -1.0f64, 1usize, 0.0f64, 0.0f64, 0.0f64);
}
