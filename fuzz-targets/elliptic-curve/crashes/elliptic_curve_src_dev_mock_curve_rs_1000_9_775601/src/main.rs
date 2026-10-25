fn main() {
    let _ = <elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::Neg>::neg(<elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::Group>::generator());
}
