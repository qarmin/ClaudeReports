use p256::Scalar;

fn main() {
    let scalar = Scalar::default();
    let _ = <p256::Scalar as ::core::ops::Add<p256::Scalar>>::add(scalar, Scalar::ZERO.sqn_vartime(1000000000usize));
}
