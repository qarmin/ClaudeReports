use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::TimeType;
use parse_zoneinfo::line::Year;
use parse_zoneinfo::table::RuleInfo;

fn main() {
    let rule_info = RuleInfo { from_year: Year::Number(0i64), to_year: Some(Year::Minimum), month: Month::April, day: DaySpec::Ordinal(0i8), time: 0i64, time_type: TimeType::Standard, time_to_add: 0i64, letters: None };
    let _ = rule_info.applies_to_year(0i64);
}
