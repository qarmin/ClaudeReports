use rust_decimal::Decimal;

fn main() {
    let _ = <rust_decimal::Decimal as rust_decimal::MathematicalOps>::norm_pdf(&Decimal::try_from(621.0f32).unwrap());
}
