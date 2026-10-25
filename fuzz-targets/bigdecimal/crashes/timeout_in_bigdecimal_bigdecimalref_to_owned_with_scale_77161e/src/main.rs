use bigdecimal::BigDecimal;

fn main() {
    let big_decimal = <bigdecimal::BigDecimal as bigdecimal::Signed>::abs_sub(&BigDecimal::from(0i16), &BigDecimal::from(0i8));
    let big_decimal2 = big_decimal.normalized();
    let big_decimal_ref = big_decimal2.to_ref();
    let _ = big_decimal_ref.to_owned_with_scale(1000000000i64);
}
