fn main() {
    let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64, 1u64, 1u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(&mut fiat_p448_tight_field_element, &fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([u64::MAX, 0u64, 0u64, 0u64, 0u64, 0u64, 10000000000000000000u64, 255u64]), &fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([255u64, 79u64, 1000u64, 8u64, 4294967296u64, 7u64, 96u64, u64::MAX]));
}
