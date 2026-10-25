use jiff::SignedDuration;
use jiff::Zoned;

fn main() {
    let _ = <jiff::Zoned as ::core::ops::Sub<jiff::SignedDuration>>::sub(Zoned::UNIX_EPOCH, SignedDuration::MIN);
}
