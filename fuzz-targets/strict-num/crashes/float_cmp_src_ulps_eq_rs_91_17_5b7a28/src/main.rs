use strict_num::PositiveF64;

fn main() {
    let _ = <strict_num::PositiveF64 as strict_num::ApproxEqUlps>::approx_eq_ulps(&PositiveF64::new(1.0f64).unwrap(), &PositiveF64::ZERO, i64::MIN);
}
