fn main() {
    let generic_fraction = <fraction::prelude::GenericFraction<u16> as fraction::Bounded>::min_value();
    let generic_fraction2 = generic_fraction.fract();
    let _ = generic_fraction2.sqrt(1000000000u32);
}
