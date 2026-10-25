fn main() {
    let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p448_solinas_64::fiat_p448_sub(&mut fiat_p448_loose_field_element, &fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 1u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64]), &fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([0u64, 0u64, u64::MAX, 3342601094461837497u64, 4418826780289787915u64, 12191843333304873439u64, 7u64, 17843022385921707698u64]));
}
