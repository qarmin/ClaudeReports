fn main() {
    let mut fiat_p521_tight_field_element = fiat_crypto::p521_32::fiat_p521_tight_field_element([0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
    fiat_crypto::p521_32::fiat_p521_carry_square(&mut fiat_p521_tight_field_element, &fiat_crypto::p521_32::fiat_p521_loose_field_element([0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, u32::MAX, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32]));
}
