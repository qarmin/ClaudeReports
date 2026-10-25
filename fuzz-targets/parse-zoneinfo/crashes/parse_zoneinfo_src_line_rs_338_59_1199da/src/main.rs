use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::Weekday;

fn main() {
    let day_spec = DaySpec::LastOnOrBefore(Weekday::Sunday, i8::MAX);
    let _ = day_spec.to_concrete_day(0i64, Month::March);
}
