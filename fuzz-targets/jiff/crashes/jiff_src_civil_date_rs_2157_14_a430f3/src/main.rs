use jiff::SignedDuration;
use jiff::civil::Date;
use jiff::civil::DateTime;

fn main() {
    let date_time = DateTime::from(DateTime::MIN);
    let _ = <jiff::civil::Date as ::core::ops::Add<jiff::SignedDuration>>::add(Date::from(date_time), SignedDuration::new(1000000000000000000i64, 0i32));
}
