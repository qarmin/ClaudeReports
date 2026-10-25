use jiff::SignedDuration;
use jiff::tz::Offset;

fn main() {
    let _ = <jiff::tz::Offset as ::core::ops::Add<jiff::SignedDuration>>::add(Offset::from_hours(0i8).unwrap(), SignedDuration::MIN);
}
