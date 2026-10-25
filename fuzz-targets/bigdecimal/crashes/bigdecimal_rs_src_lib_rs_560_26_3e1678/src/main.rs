use bigdecimal::Context;
use bigdecimal::RoundingMode;

fn main() {
    let context = Context::default();
    let context2 = context.with_rounding_mode(RoundingMode::Down);
    let context3 = context2.with_precision(std::num::NonZero::new(u64::MAX).unwrap());
    let big_decimal = <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u64(1000u64).unwrap();
    let _ = context3.round_decimal_ref::<bigdecimal::BigDecimalRef>(big_decimal.to_ref());
}
