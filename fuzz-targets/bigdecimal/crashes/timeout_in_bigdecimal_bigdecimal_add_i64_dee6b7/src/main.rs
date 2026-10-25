use bigdecimal::BigDecimal;
use bigdecimal::RoundingMode;

fn main() {
    let _ = <bigdecimal::BigDecimal as ::core::ops::Add<i64>>::add(BigDecimal::from(1i32).with_scale_round(-2535105207928642425i64, RoundingMode::Up), 18i64);
}
