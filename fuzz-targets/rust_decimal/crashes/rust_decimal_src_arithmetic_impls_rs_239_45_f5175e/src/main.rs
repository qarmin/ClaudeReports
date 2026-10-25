use rust_decimal::Decimal;

fn main() {
    let u32_v = Decimal::MAX_SCALE;
    let _ = <rust_decimal::Decimal as ::core::ops::Div<rust_decimal::Decimal>>::div(Decimal::from(u32_v), Decimal::try_from(0.0f64).unwrap());
}
