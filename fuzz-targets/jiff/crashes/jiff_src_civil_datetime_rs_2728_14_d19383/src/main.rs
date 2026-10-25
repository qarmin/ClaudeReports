use jiff::SignedDuration;
use jiff::Timestamp;
use jiff::tz::Offset;

fn main() {
    let _ = <jiff::civil::DateTime as ::core::ops::Sub<jiff::SignedDuration>>::sub(Offset::ZERO.to_datetime(Timestamp::UNIX_EPOCH), SignedDuration::from_micros(-1000000000000000000i64));
}
