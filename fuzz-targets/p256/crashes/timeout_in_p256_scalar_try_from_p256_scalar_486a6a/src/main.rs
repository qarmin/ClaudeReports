use p256::NistP256;
use p256::Scalar;

fn main() {
    let _ = NistP256;
    let _ = Scalar::try_from(Scalar::from(0u32).sqn_vartime(usize::MAX));
}
