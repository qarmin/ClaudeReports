fn main() {
    let mut fiat_p448_tight_field_element = fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 1u64, 0u64]);
    fiat_crypto::p448_solinas_64::fiat_p448_carry_mul(&mut fiat_p448_tight_field_element, &fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64, u64::MAX, 68u64, 14988368381062527468u64, 3383299671908264572u64, 0u64, 16u64]), &fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([2u64, 1u64, 18446744073709551614u64, 15035455111019105998u64, u64::MAX, u64::MAX, 13170709496967601630u64, 1u64]));
}
