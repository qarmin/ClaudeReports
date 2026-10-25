use elliptic_curve::dev::mock_curve::Scalar;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::Scalar as elliptic_curve::Field>::sqrt(&Scalar::from(1u64));
}
