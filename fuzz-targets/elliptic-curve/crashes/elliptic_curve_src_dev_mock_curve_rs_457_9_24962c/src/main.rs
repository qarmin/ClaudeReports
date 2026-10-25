use elliptic_curve::dev::mock_curve::AffinePoint;
use elliptic_curve::dev::mock_curve::Scalar;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::AffineCoordinates>::x_is_odd(&AffinePoint::FixedBaseOutput(Scalar::default()));
}
