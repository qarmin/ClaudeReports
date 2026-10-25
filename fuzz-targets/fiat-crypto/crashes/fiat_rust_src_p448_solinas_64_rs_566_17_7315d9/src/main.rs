fn main() {
    let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p448_solinas_64::fiat_p448_add(&mut fiat_p448_loose_field_element, &fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([1u64, 0u64, u64::MAX, u64::MAX, 33u64, 8u64, 255u64, 6783989175344568052u64]), &fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([1071995610357239386u64, 8u64, 16183568768227408170u64, 33u64, 4674527400073804740u64, 7791641835520423956u64, 58u64, 7u64]));
}
