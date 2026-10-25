fn main() {
    let big_decimal = <bigdecimal::BigDecimal as ::core::ops::Mul<i64>>::mul(<bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u64(10000000000000000000u64).unwrap(), 65535i64);
    let big_decimal_ref = big_decimal.to_ref();
    let _ = big_decimal_ref.exp();
}
