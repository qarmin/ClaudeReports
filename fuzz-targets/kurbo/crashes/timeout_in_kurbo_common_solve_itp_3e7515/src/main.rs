fn main() {
    let _ = kurbo::common::solve_itp(|_| 1 as f64, 0.0f64, 1.0f64, -0.0f64, 1usize, 0.0f64, f64::MIN, 0.0f64);
}
