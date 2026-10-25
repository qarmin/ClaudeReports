use elliptic_curve::dev::mock_curve::Scalar;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::Scalar as elliptic_curve::Field>::sqrt_ratio(&Scalar::default(), &Scalar::from(<elliptic_curve::ScalarValue<elliptic_curve::dev::mock_curve::MockCurve>>::ZERO));
}
