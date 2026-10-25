use rust_decimal::Decimal;

fn main() {
    let _ = <rust_decimal::Decimal as rust_decimal::MathematicalOps>::powu(&Decimal::ONE_THOUSAND, 1000000000000000000u64);
}
