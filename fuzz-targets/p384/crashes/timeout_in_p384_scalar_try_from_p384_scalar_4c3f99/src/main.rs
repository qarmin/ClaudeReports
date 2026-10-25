use p384::Scalar;

fn main() {
    let _ = Scalar::try_from(Scalar::ONE.sqn_vartime(10000000000000000000usize));
}
