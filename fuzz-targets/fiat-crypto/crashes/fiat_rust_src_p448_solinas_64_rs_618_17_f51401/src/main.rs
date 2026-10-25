fn main() {
    let mut fiat_p448_loose_field_element = fiat_crypto::p448_solinas_64::fiat_p448_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p448_solinas_64::fiat_p448_opp(&mut fiat_p448_loose_field_element, &fiat_crypto::p448_solinas_64::fiat_p448_tight_field_element([10000000000000000000u64, 8u64, 8965607643552440008u64, 0u64, 0u64, 731325735078940678u64, 10006887957558925050u64, 72381077612263818u64]));
}
