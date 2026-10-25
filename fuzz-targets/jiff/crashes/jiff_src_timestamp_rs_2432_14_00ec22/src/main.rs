use jiff::Span;
use jiff::Timestamp;

fn main() {
    let span = <jiff::Span as ::core::ops::Neg>::neg(Span::new());
    let _ = <jiff::Timestamp as ::core::ops::Sub<jiff::Span>>::sub(Timestamp::MIN, span.hours::<i64>(1i64));
}
