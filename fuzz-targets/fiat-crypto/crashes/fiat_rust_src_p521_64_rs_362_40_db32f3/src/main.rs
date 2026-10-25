fn main() {
    let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p521_64::fiat_p521_carry_mul(&mut fiat_p521_tight_field_element, &fiat_crypto::p521_64::fiat_p521_loose_field_element([1u64, 0u64, 1u64, u64::MAX, u64::MAX, 14u64, 1u64, 1u64, 11u64]), &fiat_crypto::p521_64::fiat_p521_loose_field_element([18446744073709551614u64, 4294967296u64, 4294967295u64, 1u64, 4294967295u64, 255u64, 8u64, 14u64, 65535u64]));
}
