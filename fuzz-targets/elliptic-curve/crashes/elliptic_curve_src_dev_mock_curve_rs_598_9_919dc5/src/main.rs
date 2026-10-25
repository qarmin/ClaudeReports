use elliptic_curve::dev::mock_curve::AffinePoint;

fn main() {
    let scalar_value = <elliptic_curve::ScalarValue<elliptic_curve::dev::mock_curve::MockCurve>>::ONE;
    let string = scalar_value.to_string();
    let _ = <elliptic_curve::dev::mock_curve::AffinePoint as ::core::ops::Mul<elliptic_curve::NonZeroScalar<elliptic_curve::dev::mock_curve::MockCurve>>>::mul(AffinePoint::Generator, <elliptic_curve::NonZeroScalar<elliptic_curve::dev::mock_curve::MockCurve> as ::core::str::FromStr>::from_str(&string).unwrap());
}
