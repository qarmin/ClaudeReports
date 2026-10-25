fn main() {
    let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
    fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(&mut fiat_p448_tight_field_element, &fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32, 0u32]), &fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]));
}
