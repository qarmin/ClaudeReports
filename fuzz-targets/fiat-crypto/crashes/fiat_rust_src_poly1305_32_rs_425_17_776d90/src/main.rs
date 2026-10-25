fn main() {
    let mut fiat_poly1305_loose_field_element = fiat_crypto::poly1305_32::fiat_poly1305_loose_field_element([1u32, 0u32, 0u32, 0u32, 0u32]);
    fiat_crypto::poly1305_32::fiat_poly1305_opp(&mut fiat_poly1305_loose_field_element, &fiat_crypto::poly1305_32::fiat_poly1305_tight_field_element([0u32, 0u32, 0u32, 778121783u32, 0u32]));
}
