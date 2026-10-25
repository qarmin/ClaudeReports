use rust_decimal::Decimal;

fn main() {
    let decimal = Decimal::from(usize::MAX);
    let _ = <rust_decimal::Decimal as rust_decimal::MathematicalOps>::exp(&decimal);
}
