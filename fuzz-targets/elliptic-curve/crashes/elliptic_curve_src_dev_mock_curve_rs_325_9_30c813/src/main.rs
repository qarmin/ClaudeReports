fn main() {
    let scalar_value = <elliptic_curve::ScalarValue<elliptic_curve::dev::mock_curve::MockCurve> as ::core::convert::From<u64>>::from(1u64);
    let string = scalar_value.to_string();
    let _ = <elliptic_curve::NonZeroScalar<elliptic_curve::dev::mock_curve::MockCurve> as elliptic_curve::ops::Invert>::invert_vartime(&<elliptic_curve::NonZeroScalar<elliptic_curve::dev::mock_curve::MockCurve> as ::core::str::FromStr>::from_str(&string).unwrap());
}
