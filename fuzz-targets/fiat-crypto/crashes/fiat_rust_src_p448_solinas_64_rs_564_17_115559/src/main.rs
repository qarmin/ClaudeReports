fn main() {
    let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p448_solinas_64::fiat_p448_add(&mut fiat_p448_loose_field_element, &fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([10000000000000000000u64, 0u64, 66u64, 0u64, 13628641190857162675u64, 25u64, 11971366163859023337u64, 12907745166445320433u64]), &fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([18446744073709551614u64, 4600271130141445088u64, u64::MAX, u64::MAX, 1u64, 7070801945689275461u64, u64::MAX, 46u64]));
}
