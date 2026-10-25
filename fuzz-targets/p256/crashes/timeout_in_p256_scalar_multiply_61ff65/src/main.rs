use p256::Scalar;

fn main() {
    let scalar = <p256::Scalar as ::core::ops::Shr<usize>>::shr(Scalar::from(0u32), 0usize);
    let _ = scalar.multiply(&Scalar::default().sqn_vartime(usize::MAX));
}
