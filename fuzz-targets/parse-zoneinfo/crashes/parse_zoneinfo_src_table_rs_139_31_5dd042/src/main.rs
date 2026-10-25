use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::TimeType;
use parse_zoneinfo::line::Year;
use parse_zoneinfo::table::RuleInfo;

fn main() {
    let rule_info = RuleInfo { from_year: Year::Maximum, to_year: None, month: Month::August, day: DaySpec::Ordinal(0i8), time: 0i64, time_type: TimeType::Wall, time_to_add: 0i64, letters: None };
    let _ = rule_info.absolute_datetime(0i64, 1i64, i64::MAX);
}
