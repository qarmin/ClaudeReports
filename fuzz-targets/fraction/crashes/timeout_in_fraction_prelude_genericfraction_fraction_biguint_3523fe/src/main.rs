fn main() {
    let _ = <fraction::prelude::GenericFraction<fraction::BigUint> as fraction::ToPrimitive>::to_u64(&<fraction::prelude::GenericFraction<fraction::BigInt> as fraction::Zero>::zero().sqrt(1000000000u32));
}
