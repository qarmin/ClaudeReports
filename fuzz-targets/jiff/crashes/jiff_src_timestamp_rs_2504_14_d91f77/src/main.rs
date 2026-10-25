use jiff::SignedDuration;
use jiff::Timestamp;

fn main() {
    let _ = <jiff::Timestamp as ::core::ops::Sub<jiff::SignedDuration>>::sub(Timestamp::from_second(0i64).unwrap(), SignedDuration::MIN);
}
