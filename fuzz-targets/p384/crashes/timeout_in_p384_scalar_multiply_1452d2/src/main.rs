use p384::Scalar;

fn main() {
    let scalar = Scalar::ONE;
    let _ = scalar.multiply(&Scalar::default().sqn_vartime(2147483648usize));
}
