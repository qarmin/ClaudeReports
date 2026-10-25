use jiff::Span;
use jiff::civil::Date;
use jiff::civil::DateTime;

fn main() {
    let span = Span::try_from(Date::MAX.duration_until(Date::MIN)).unwrap();
    let span2 = span.days::<i64>(0i64);
    let span3 = span2.hours::<i64>(0i64);
    let _ = <jiff::civil::DateTime as ::core::ops::Add<jiff::Span>>::add(DateTime::default(), span3);
}
