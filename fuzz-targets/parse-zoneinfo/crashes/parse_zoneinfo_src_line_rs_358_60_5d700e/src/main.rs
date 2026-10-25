use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::Weekday;

fn main() {
    let month = Month::May;
    let day_spec = DaySpec::FirstOnOrAfter(Weekday::Friday, i8::MAX);
    let _ = day_spec.to_concrete_day(0i64, month);
}
