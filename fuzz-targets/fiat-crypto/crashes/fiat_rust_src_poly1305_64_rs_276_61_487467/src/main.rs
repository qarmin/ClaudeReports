fn main() {
    let mut fiat_poly1305_tight_field_element = fiat_crypto::poly1305_64::fiat_poly1305_tight_field_element([0u64, 0u64, 1u64]);
    fiat_crypto::poly1305_64::fiat_poly1305_carry_mul(&mut fiat_poly1305_tight_field_element, &fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([0u64, 1u64, 0u64]), &fiat_crypto::poly1305_64::fiat_poly1305_loose_field_element([0u64, 0u64, 2709226863620216900u64]));
}
