use jiff::SignedDuration;
use jiff::Timestamp;
use jiff::Zoned;

fn main() {
    let _ = <jiff::Timestamp as ::core::ops::Add<jiff::SignedDuration>>::add(Timestamp::from(Zoned::default()), SignedDuration::MAX);
}
