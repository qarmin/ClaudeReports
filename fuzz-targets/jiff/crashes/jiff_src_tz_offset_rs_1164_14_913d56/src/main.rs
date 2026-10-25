use jiff::SignedDuration;
use jiff::Span;
use jiff::tz::Offset;

fn main() {
    let _ = <jiff::tz::Offset as ::core::ops::Add<jiff::Span>>::add(Offset::from_seconds(0i32).unwrap(), Span::try_from(SignedDuration::from_hours(91i64)).unwrap());
}
