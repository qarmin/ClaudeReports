use p384::Scalar;

fn main() {
    let scalar = Scalar::try_from(Scalar::ZERO.sqn_vartime(65536usize)).unwrap();
    let scalar2 = scalar.add(&Scalar::default());
    let _ = scalar2.sqn_vartime(4294967296usize);
}
