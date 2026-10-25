use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::TimeType;
use parse_zoneinfo::line::Weekday;
use parse_zoneinfo::line::Year;
use parse_zoneinfo::table::RuleInfo;

fn main() {
    let day_spec = DaySpec::LastOnOrBefore(Weekday::Saturday, -100i8);
    let rule_info = RuleInfo { from_year: Year::Number(2043728258908512197i64), to_year: Some(Year::Number(62i64)), month: Month::February, day: day_spec, time: -7833470800563864112i64, time_type: TimeType::Wall, time_to_add: i64::MAX, letters: Some("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"56\" height=\"NaN\" viewBox=\"0 0 1e9 142\"><defs><linearGradient id=\"g\"><stop offset=\"0\" stop-color=\"blue\"/><stop offset=\"1\" stop-color=\"yellow\" stop-opacity=\"0.5\"/></linearGradient><clipPath id=\"c\"><circle cx=\"50\" cy=\"50\" r=\"40\"".to_string()) };
    let _ = rule_info.absolute_datetime(i64::MAX, 7i64, 4473909183253428745i64);
}
