fn main() {
    let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p521_64::fiat_p521_sub(&mut fiat_p521_loose_field_element, &fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, u64::MAX, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]), &fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]));
}
