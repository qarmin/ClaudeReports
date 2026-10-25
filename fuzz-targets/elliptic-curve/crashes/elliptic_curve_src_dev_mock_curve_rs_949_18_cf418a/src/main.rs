use elliptic_curve::dev::mock_curve::Scalar;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::Mul<elliptic_curve::dev::mock_curve::Scalar>>::mul(<elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::Group>::identity(), Scalar::default());
}
