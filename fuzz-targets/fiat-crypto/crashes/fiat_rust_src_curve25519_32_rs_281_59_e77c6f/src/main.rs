fn main() {
    let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_32::fiat_25519_tight_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
    fiat_crypto::curve25519_32::fiat_25519_carry_mul(&mut fiat_25519_tight_field_element, &fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]), &fiat_crypto::curve25519_32::fiat_25519_loose_field_element([0u32, 0u32, 1000000000u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32]));
}
