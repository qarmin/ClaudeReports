fn main() {
    let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p448_solinas_64::fiat_p448_sub(&mut fiat_p448_loose_field_element, &fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 1u64]), &fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64, 0u64, u64::MAX, 2u64, 1u64, 65535u64, 58u64]));
}
