use p256::Scalar;

fn main() {
    let _ = <p256::Scalar as ::core::ops::Mul<p256::Scalar>>::mul(Scalar::ZERO, Scalar::default().sqn_vartime(usize::MAX));
}
