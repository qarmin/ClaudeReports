use bigdecimal::BigDecimal;

fn main() {
    let _ = <bigdecimal::BigDecimal as ::core::ops::Sub<i32>>::sub(BigDecimal::from(1i8).with_scale(1000000000i64), 0i32);
}
