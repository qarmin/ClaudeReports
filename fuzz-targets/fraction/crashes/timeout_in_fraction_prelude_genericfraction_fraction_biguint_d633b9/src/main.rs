fn main() {
    let generic_fraction = <fraction::prelude::GenericFraction<fraction::BigUint> as ::core::convert::From<u64>>::from(0u64).sqrt(1000000000u32);
    let _ = generic_fraction.denom();
}
