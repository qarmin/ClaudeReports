use p256::Scalar;

fn main() {
    let _ = <p256::Scalar as ::core::ops::Shr<usize>>::shr(<p256::Scalar as ::core::ops::Add<p256::Scalar>>::add(Scalar::from(Scalar::from(64u64)).sqn_vartime(usize::MAX), Scalar::from(255u32)), 2147483648usize);
}
