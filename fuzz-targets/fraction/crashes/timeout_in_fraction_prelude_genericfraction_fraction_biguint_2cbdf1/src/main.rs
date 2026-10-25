fn main() {
    let generic_fraction = <fraction::prelude::GenericFraction<fraction::BigUint> as ::core::convert::From<i32>>::from(0i32);
    let _ = generic_fraction.sqrt_abs(1000000000u32);
}
