use jiff::SignedDuration;
use jiff::civil::Date;

fn main() {
    let _ = <jiff::civil::Date as ::core::ops::Sub<jiff::SignedDuration>>::sub(Date::ZERO, SignedDuration::from_micros(1000000000000000000i64));
}
