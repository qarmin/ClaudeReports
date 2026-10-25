fn main() {
    let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_32::fiat_p448_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32]);
    fiat_crypto::p448_solinas_32::fiat_p448_carry_mul(&mut fiat_p448_tight_field_element, &fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32, 0u32, u32::MAX, 0u32, 1000000000u32, 0u32, 0u32, 0u32, 3553025663u32]), &fiat_crypto::p448_solinas_32::fiat_p448_loose_field_element([3692082540u32, 0u32, 0u32, 0u32, 1000000000u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1073741824u32, 0u32]));
}
