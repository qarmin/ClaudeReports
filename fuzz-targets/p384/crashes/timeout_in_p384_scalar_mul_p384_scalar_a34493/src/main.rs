use p384::Scalar;

fn main() {
    let _ = <p384::Scalar as ::core::ops::Mul<p384::Scalar>>::mul(Scalar::from(0u64).sqn_vartime(10000000000000000000usize), Scalar::from(16u32));
}
