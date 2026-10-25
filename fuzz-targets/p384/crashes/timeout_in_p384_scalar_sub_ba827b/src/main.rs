use p384::Scalar;

fn main() {
    let scalar = Scalar::from_u64(0u64);
    let _ = scalar.sub(&Scalar::from(0u64).sqn_vartime(1000000000usize));
}
