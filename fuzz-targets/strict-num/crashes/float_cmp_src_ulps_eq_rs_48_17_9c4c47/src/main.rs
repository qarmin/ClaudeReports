use strict_num::PositiveF32;

fn main() {
    let _ = <strict_num::PositiveF32 as strict_num::ApproxEqUlps>::approx_eq_ulps(&PositiveF32::new(1.0f32).unwrap(), &PositiveF32::default(), i32::MIN);
}
