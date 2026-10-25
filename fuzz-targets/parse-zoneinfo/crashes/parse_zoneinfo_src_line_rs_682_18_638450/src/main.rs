use parse_zoneinfo::line::ChangeTime;
use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::Weekday;
use parse_zoneinfo::line::Year;

fn main() {
    let day_spec = DaySpec::LastOnOrBefore(Weekday::Sunday, 0i8);
    let year = Year::Maximum;
    let change_time = ChangeTime::UntilDay(year, Month::August, day_spec);
    let _ = change_time.year();
}
