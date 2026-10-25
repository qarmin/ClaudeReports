fn main() {
    let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p448_solinas_64::fiat_p448_carry_square(&mut fiat_p448_tight_field_element, &fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64, 10000000000000000000u64, 7331314549758190858u64, 7u64, 65535u64, 7u64, 70u64]));
}
