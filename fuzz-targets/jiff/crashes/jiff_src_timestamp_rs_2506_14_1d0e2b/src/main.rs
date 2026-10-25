use jiff::SignedDuration;
use jiff::Timestamp;

fn main() {
    let _ = <jiff::Timestamp as ::core::ops::Sub<jiff::SignedDuration>>::sub(Timestamp::constant(0i64, 0i32), SignedDuration::MAX);
}
