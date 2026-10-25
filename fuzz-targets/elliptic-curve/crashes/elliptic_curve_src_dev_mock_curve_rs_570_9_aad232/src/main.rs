use elliptic_curve::dev::mock_curve::AffinePoint;
use elliptic_curve::dev::mock_curve::Scalar;

fn main() {
    let scalar = Scalar::try_from(Scalar::from(<elliptic_curve::ScalarValue<elliptic_curve::dev::mock_curve::MockCurve>>::ZERO)).unwrap();
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as ::core::ops::Mul<elliptic_curve::dev::mock_curve::Scalar>>::mul(AffinePoint::FixedBaseOutput(scalar), scalar);
}
