use elliptic_curve::dev::mock_curve::AffinePoint;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::Sub<elliptic_curve::dev::mock_curve::AffinePoint>>::sub(<elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::Group>::generator(), AffinePoint::default());
}
