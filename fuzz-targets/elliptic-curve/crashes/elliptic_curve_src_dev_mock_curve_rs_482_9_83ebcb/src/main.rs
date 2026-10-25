fn main() {
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::CurveAffine>::to_curve(&<elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::CurveAffine>::generator());
}
