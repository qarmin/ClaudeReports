use jiff::Timestamp;
use jiff::civil::DateTime;

fn main() {
    let _ = <jiff::civil::DateTime as ::core::ops::Sub<jiff::Span>>::sub(DateTime::default(), <jiff::Timestamp as ::core::ops::Sub<jiff::Timestamp>>::sub(Timestamp::new(0i64, 0i32).unwrap(), Timestamp::MIN));
}
