use jiff::Timestamp;
use jiff::civil::Date;
use jiff::civil::DateTime;
use jiff::civil::Time;

fn main() {
    let timestamp = Timestamp::constant(0i64, 0i32);
    let span = <jiff::civil::DateTime as ::core::ops::Sub<jiff::civil::DateTime>>::sub(DateTime::from(Date::default()), DateTime::from_parts(Date::MAX, Time::MIN));
    let _ = <jiff::Timestamp as ::core::ops::Add<jiff::Span>>::add(timestamp, span);
}
