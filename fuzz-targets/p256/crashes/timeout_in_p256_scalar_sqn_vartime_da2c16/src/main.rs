use p256::Scalar;

fn main() {
    let scalar = Scalar::ONE;
    let _ = scalar.sqn_vartime(1000000000usize);
}
