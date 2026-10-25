use jiff::SignedDuration;
use jiff::tz::Offset;

fn main() {
    let _ = <jiff::tz::Offset as ::core::ops::Sub<jiff::SignedDuration>>::sub(Offset::MAX, SignedDuration::MAX);
}
