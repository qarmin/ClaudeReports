use curve25519_dalek::RistrettoPoint;
use curve25519_dalek::Scalar;

fn main() {
    let _ = RistrettoPoint::mul_base(&<curve25519_dalek::Scalar as ::core::ops::Sub<curve25519_dalek::Scalar>>::sub(Scalar::from(0u32), Scalar::from_bits([0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 64u8])));
}
