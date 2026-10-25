fn main() {
    let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 1u64]);
    fiat_crypto::p448_solinas_64::fiat_p448_opp(&mut fiat_p448_loose_field_element, &fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 1000000000000000000u64, 0u64, 0u64, 0u64]));
}
