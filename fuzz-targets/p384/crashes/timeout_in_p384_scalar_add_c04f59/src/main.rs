use p384::Scalar;

fn main() {
    let scalar = Scalar::default();
    let scalar2 = scalar.neg();
    let _ = scalar.add(&scalar2.sqn_vartime(1000000000usize));
}
