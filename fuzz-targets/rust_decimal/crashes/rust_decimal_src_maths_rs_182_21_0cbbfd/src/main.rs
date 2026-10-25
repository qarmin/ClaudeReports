use rust_decimal::Decimal;

fn main() {
    let _ = <rust_decimal::Decimal as rust_decimal::MathematicalOps>::exp_with_tolerance(&Decimal::from(100u8), Decimal::PI);
}
