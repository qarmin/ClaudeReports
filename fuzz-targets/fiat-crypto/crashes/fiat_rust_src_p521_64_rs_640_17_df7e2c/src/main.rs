fn main() {
    let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 1u64, 0u64]);
    fiat_crypto::p521_64::fiat_p521_opp(&mut fiat_p521_loose_field_element, &fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 10000000000000000000u64, 255u64, 255u64, 1u64, 16u64, 17740083202064477018u64, 1084218626996030245u64]));
}
