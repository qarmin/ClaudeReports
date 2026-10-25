use jiff::SignedDuration;
use jiff::Zoned;

fn main() {
    let _ = <jiff::Zoned as ::core::ops::Add<jiff::SignedDuration>>::add(Zoned::default(), SignedDuration::from_secs(-1000000000000000000i64));
}
