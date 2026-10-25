use elliptic_curve::dev::mock_curve::ProjectivePoint;

fn main() {
    let projective_point = ProjectivePoint::Generator;
    let _ = <elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::CurveGroup>::to_affine(&projective_point);
}
