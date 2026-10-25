use jiff::Span;
use jiff::civil::ISOWeekDate;

fn main() {
    let _ = <jiff::civil::Date as ::core::ops::Add<jiff::Span>>::add(ISOWeekDate::MAX.date(), Span::new().try_years::<i64>(1i64).unwrap());
}
