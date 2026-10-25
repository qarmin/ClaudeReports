use rust_decimal::Decimal;

fn main() {
    let _ = <rust_decimal::Decimal as rust_decimal::MathematicalOps>::powd(&Decimal::from(8i16), Decimal::ONE_THOUSAND);
}
