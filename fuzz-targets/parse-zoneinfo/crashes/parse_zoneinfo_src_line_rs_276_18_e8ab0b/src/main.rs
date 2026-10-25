use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::TimeType;
use parse_zoneinfo::line::Weekday;
use parse_zoneinfo::line::Year;
use parse_zoneinfo::table::RuleInfo;

fn main() {
    let rule_info = RuleInfo { from_year: Year::Minimum, to_year: None, month: Month::March, day: DaySpec::FirstOnOrAfter(Weekday::Tuesday, -10i8), time: 0i64, time_type: TimeType::Standard, time_to_add: 0i64, letters: None };
    let _ = rule_info.absolute_datetime(0i64, 0i64, 0i64);
}
