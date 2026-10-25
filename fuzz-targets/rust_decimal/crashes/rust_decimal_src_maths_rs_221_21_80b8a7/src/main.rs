use rust_decimal::Decimal;

fn main() {
    let _ = <rust_decimal::Decimal as rust_decimal::MathematicalOps>::norm_cdf(&Decimal::from(-10000i16));
}
