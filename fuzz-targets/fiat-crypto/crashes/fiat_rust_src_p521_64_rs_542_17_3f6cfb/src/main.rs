fn main() {
    let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p521_64::fiat_p521_carry(&mut fiat_p521_tight_field_element, &fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, u64::MAX, u64::MAX, u64::MAX, u64::MAX, 65535u64, 77u64, 3718242967858612509u64, 9575840534231439296u64]));
}
