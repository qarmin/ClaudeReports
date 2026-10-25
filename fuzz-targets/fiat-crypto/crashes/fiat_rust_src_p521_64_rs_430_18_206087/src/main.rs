fn main() {
    let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64, 1u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p521_64::fiat_p521_carry_square(&mut fiat_p521_tight_field_element, &fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 0u64, 0u64, u64::MAX, 65535u64, 15u64, 8479369336086608358u64, 91u64, 7u64]));
}
