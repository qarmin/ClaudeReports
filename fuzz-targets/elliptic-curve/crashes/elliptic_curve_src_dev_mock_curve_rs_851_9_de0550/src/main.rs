use elliptic_curve::dev::mock_curve::ProjectivePoint;

fn main() {
    let projective_point = <elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::Group>::identity();
    let _ = <elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::Sub<elliptic_curve::dev::mock_curve::ProjectivePoint>>::sub(ProjectivePoint::Identity, projective_point);
}
