use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::Weekday;

fn main() {
    let day_spec = DaySpec::Last(Weekday::Tuesday);
    let _ = day_spec.to_concrete_day(i64::MIN, Month::January);
}
