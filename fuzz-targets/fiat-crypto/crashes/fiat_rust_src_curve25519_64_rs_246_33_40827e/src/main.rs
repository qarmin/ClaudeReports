fn main() {
    let mut fiat_25519_tight_field_element = fiat_crypto::curve25519_64::fiat_25519_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::curve25519_64::fiat_25519_carry_mul(&mut fiat_25519_tight_field_element, &fiat_crypto::curve25519_64::fiat_25519_loose_field_element([0u64, 0u64, 10000000000000000000u64, 256u64, 18446744073709551614u64]), &fiat_crypto::curve25519_64::fiat_25519_loose_field_element([u64::MAX, 67u64, 14u64, 26u64, 1000u64]));
}
