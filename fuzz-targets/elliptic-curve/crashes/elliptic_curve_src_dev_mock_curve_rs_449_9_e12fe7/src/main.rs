use elliptic_curve::dev::mock_curve::AffinePoint;
use elliptic_curve::dev::mock_curve::Scalar;

fn main() {
    let scalar = Scalar::try_from(Scalar::default()).unwrap();
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::AffineCoordinates>::x(&AffinePoint::FixedBaseOutput(scalar));
}
