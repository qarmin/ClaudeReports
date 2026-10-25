use bigdecimal::BigDecimal;
use bigdecimal::Context;
use bigdecimal::RoundingMode;

fn main() {
    let big_decimal = <bigdecimal::BigDecimal as ::core::ops::Neg>::neg(BigDecimal::from(0i32));
    let big_decimal2 = big_decimal.with_scale_round(0i64, RoundingMode::HalfEven);
    let big_decimal3 = big_decimal2.with_scale(0i64);
    let big_decimal_ref = big_decimal3.to_ref();
    let _ = big_decimal_ref.sqrt_copysign_with_context(&Context::default().with_precision(std::num::NonZero::new(u64::MAX).unwrap()));
}
