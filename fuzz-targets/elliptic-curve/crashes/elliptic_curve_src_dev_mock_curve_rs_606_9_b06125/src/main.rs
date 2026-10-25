fn main() {
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as ::core::ops::Neg>::neg(<elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::CurveAffine>::identity());
}
