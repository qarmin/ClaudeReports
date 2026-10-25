use rust_decimal::Decimal;

fn main() {
    let _ = <rust_decimal::Decimal as rust_decimal::MathematicalOps>::exp_with_tolerance(&<rust_decimal::Decimal as ::core::ops::Neg>::neg(Decimal::from(1000000000u64)), Decimal::ZERO);
}
