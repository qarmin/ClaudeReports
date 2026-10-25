use elliptic_curve::dev::mock_curve::ProjectivePoint;
use elliptic_curve::dev::mock_curve::Scalar;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::Group>::double(&ProjectivePoint::FixedBaseOutput(Scalar::from(<elliptic_curve::ScalarValue<elliptic_curve::dev::mock_curve::MockCurve> as ::core::default::Default>::default())));
}
