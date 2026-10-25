use jiff::civil::Date;
use jiff::civil::DateTime;

fn main() {
    let _ = <jiff::civil::DateTime as ::core::ops::Add<jiff::SignedDuration>>::add(DateTime::MIN, Date::MAX.duration_until(Date::ZERO));
}
