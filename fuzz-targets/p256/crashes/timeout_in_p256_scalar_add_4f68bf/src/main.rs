use p256::Scalar;

fn main() {
    let scalar = Scalar::ONE;
    let _ = scalar.add(&Scalar::default().sqn_vartime(1000000000000000000usize));
}
