use bigdecimal::BigDecimal;
use bigdecimal::RoundingMode;

fn main() {
    let rounding_mode = RoundingMode::Floor;
    let _ = <bigdecimal::BigDecimal as bigdecimal::ToPrimitive>::to_i64(&BigDecimal::from(1u16).with_precision_round(std::num::NonZero::new(4294967295u64).unwrap(), rounding_mode));
}
