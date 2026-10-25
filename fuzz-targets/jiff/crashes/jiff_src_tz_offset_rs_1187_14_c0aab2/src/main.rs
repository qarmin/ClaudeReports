use jiff::tz::Offset;

fn main() {
    let _ = <jiff::tz::Offset as ::core::ops::Sub<jiff::Span>>::sub(Offset::MAX, Offset::MAX.until(Offset::UTC));
}
