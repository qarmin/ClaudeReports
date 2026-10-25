use rust_decimal::Decimal;

fn main() {
    let decimal = Decimal::MIN;
    let _ = <rust_decimal::Decimal as ::core::ops::Rem<rust_decimal::Decimal>>::rem(decimal, Decimal::ZERO);
}
