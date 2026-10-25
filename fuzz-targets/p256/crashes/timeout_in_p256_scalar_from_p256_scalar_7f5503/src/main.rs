use p256::Scalar;

fn main() {
    let scalar = <p256::Scalar as ::core::ops::Add<p256::Scalar>>::add(Scalar::from(0u64), <p256::Scalar as ::core::ops::Neg>::neg(Scalar::from(u32::MAX)).sqn_vartime(2usize));
    let scalar2 = scalar.shr_vartime(3880915770u32);
    let scalar3 = <p256::Scalar as ::core::ops::Neg>::neg(scalar2);
    let _ = Scalar::from(scalar3.sqn_vartime(2147483648usize));
}
