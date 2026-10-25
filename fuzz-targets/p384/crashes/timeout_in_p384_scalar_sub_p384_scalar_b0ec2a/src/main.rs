use p384::Scalar;

fn main() {
    let _ = <p384::Scalar as ::core::ops::Sub<p384::Scalar>>::sub(Scalar::from(0u64), Scalar::from(0u64).sqn_vartime(10000000000000000000usize));
}
