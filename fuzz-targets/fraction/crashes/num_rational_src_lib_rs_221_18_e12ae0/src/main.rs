fn main() {
    let generic_fraction = <fraction::prelude::GenericFraction<u16>>::new_neg::<u16, u16>(u16::MAX, 1u16);
    let generic_fraction2 = generic_fraction.abs();
    let _ = generic_fraction2.ceil();
}
