fn main() {
    let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::curve25519_64::fiat_25519_opp(&mut fiat_25519_loose_field_element, &fiat_crypto::curve25519_64::fiat_25519_tight_field_element([0u64, 0u64, u64::MAX, 1u64, 13u64]));
}
