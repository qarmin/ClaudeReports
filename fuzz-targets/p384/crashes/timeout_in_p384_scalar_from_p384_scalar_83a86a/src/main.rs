use p384::Scalar;

fn main() {
    let _ = Scalar::from(Scalar::from(255u64).sqn_vartime(2147483648usize).double());
}
