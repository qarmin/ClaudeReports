fn main() {
    let _ = kurbo::common::solve_itp(|_| 1 as f64, 0.0f64, 0.0f64, 0.0f64, 1000000000usize, 0.0f64, 0.0f64, 0.0f64);
}
