fn main() {
    let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p521_64::fiat_p521_opp(&mut fiat_p521_loose_field_element, &fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 1u64, 0u64, 0u64, 10000000000000000000u64, 1966608797484433845u64, 2046354619379167194u64, 4611686018427387904u64]));
}
