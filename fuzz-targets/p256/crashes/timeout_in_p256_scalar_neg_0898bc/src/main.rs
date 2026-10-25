use p256::Scalar;

fn main() {
    let scalar = Scalar::ZERO;
    let _ = <p256::Scalar as ::core::ops::Neg>::neg(scalar.sqn_vartime(10000000000000000000usize));
}
