use elliptic_curve::dev::mock_curve::ProjectivePoint;

fn main() {
    let affine_point = <elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::CurveAffine>::generator();
    let _ = <elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::Add<elliptic_curve::dev::mock_curve::AffinePoint>>::add(ProjectivePoint::default(), affine_point);
}
