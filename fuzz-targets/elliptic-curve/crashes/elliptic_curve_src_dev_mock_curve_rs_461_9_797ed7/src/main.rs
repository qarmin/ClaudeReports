use elliptic_curve::dev::mock_curve::AffinePoint;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::AffineCoordinates>::y_is_odd(&AffinePoint::default());
}
