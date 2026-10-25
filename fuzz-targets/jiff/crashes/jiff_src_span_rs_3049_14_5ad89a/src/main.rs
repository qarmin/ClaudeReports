use jiff::tz::Offset;

fn main() {
    let _ = <jiff::Span as ::core::ops::Mul<i64>>::mul(<jiff::tz::Offset as ::core::ops::Sub<jiff::tz::Offset>>::sub(Offset::UTC, Offset::from_seconds(0i32).unwrap()).minutes::<i64>(1i64), -1000000000000000000i64);
}
