use p384::Scalar;

fn main() {
    let _ = <p384::Scalar as ::core::ops::Add<p384::Scalar>>::add(Scalar::from(0u32), Scalar::default().sqn_vartime(1000000000usize));
}
