use bigdecimal::BigDecimal;
use bigdecimal::Context;
use bigdecimal::RoundingMode;

fn main() {
    let big_decimal = <bigdecimal::BigDecimal as ::core::ops::Mul<i16>>::mul(BigDecimal::try_from(1.0f32).unwrap(), 1i16);
    let big_decimal2 = big_decimal.sqrt().unwrap();
    let big_decimal_ref = big_decimal2.to_ref();
    let _ = Context::new(std::num::NonZero::new(u64::MAX).unwrap(), RoundingMode::HalfDown).invert::<bigdecimal::BigDecimalRef>(big_decimal_ref);
}
