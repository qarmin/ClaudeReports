use elliptic_curve::dev::mock_curve::Scalar;

fn main() {
    let _ = <elliptic_curve::dev::mock_curve::Scalar as elliptic_curve::Field>::invert(&Scalar::default());
}
