fn main() {
    let mut fiat_p521_loose_field_element = fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p521_64::fiat_p521_opp(&mut fiat_p521_loose_field_element, &fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64, 10000000000000000000u64, 67u64, 12148215332203628892u64, 14123133107686034899u64, 16268969613788349593u64, 0u64]));
}
