use elliptic_curve::dev::mock_curve::Scalar;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::Scalar as ::core::ops::Mul<elliptic_curve::dev::mock_curve::Scalar>>::mul(Scalar::default(), Scalar::default());
}
