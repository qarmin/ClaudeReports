use elliptic_curve::dev::mock_curve::ProjectivePoint;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::Add<elliptic_curve::dev::mock_curve::ProjectivePoint>>::add(ProjectivePoint::from(<elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::CurveAffine>::generator()), ProjectivePoint::from(<elliptic_curve::point::NonIdentity<elliptic_curve::dev::mock_curve::ProjectivePoint> as ::core::convert::TryFrom<elliptic_curve::dev::mock_curve::ProjectivePoint>>::try_from(<elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::Group>::generator()).unwrap()));
}
