use parse_zoneinfo::line::ChangeTime;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::Year;

fn main() {
    let change_time = ChangeTime::UntilMonth(Year::Maximum, Month::June);
    let _ = change_time.to_timestamp(0i64, 0i64);
}
