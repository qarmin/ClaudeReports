fn main() {
    let mut fiat_p521_tight_field_element = fiat_crypto::p521_64::fiat_p521_tight_field_element([0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
    fiat_crypto::p521_64::fiat_p521_carry_mul(&mut fiat_p521_tight_field_element, &fiat_crypto::p521_64::fiat_p521_loose_field_element([0u64, 10000000000000000000u64, 3391617135753145415u64, 2u64, 0u64, 2u64, 12271124293904696155u64, 16593284771299102874u64, 255u64]), &fiat_crypto::p521_64::fiat_p521_loose_field_element([4294967295u64, 255u64, 7157680848120156213u64, 0u64, 1u64, 6834338645875110267u64, 32u64, 1202058731829442497u64, 7734471760046062691u64]));
}
