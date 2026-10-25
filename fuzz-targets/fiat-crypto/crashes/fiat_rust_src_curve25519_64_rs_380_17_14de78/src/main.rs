fn main() {
    let mut fiat_25519_loose_field_element = fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64, 0u64, 1u64, 0u64, 0u64]);
    fiat_crypto::curve25519_64::fiat_25519_add(&mut fiat_25519_loose_field_element, &fiat_crypto::curve25519_64::fiat_25519_tight_field_element([1u64, 0u64, 0u64, 0u64, 0u64]), &fiat_crypto::curve25519_64::fiat_25519_tight_field_element([u64::MAX, 0u64, 0u64, 0u64, 0u64]));
}
