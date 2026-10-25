fn main() {
    let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p521_64::fiat_p521_carry(&mut fiat_p521_tight_field_element, &fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64, 10000000000000000000u64, u64::MAX, 40u64, 18446744073709551614u64, 78u64, 8u64]));
}
