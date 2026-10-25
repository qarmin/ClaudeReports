use p256::Scalar;

fn main() {
    let scalar = Scalar::from(0u64);
    let scalar2 = scalar.double();
    let _ = <p256::Scalar as ::core::ops::Sub<p256::Scalar>>::sub(scalar2.sqn_vartime(usize::MAX), Scalar::from(32327102942228415u64));
}
