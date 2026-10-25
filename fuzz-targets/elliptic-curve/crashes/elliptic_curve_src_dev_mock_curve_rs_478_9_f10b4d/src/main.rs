fn main() {
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::CurveAffine>::is_identity(&<elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::CurveAffine>::identity());
}
