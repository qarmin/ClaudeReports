use elliptic_curve::dev::mock_curve::AffinePoint;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::AffineCoordinates>::y(&<elliptic_curve::point::NonIdentity<elliptic_curve::dev::mock_curve::AffinePoint> as ::core::convert::TryFrom<elliptic_curve::dev::mock_curve::AffinePoint>>::try_from(AffinePoint::Generator).unwrap().to_point());
}
