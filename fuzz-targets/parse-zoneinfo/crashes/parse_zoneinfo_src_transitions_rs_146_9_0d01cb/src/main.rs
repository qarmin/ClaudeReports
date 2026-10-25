use parse_zoneinfo::transitions::FixedTimespan;

fn main() {
    let fixed_timespan = FixedTimespan { utc_offset: i64::MAX, dst_offset: 1i64, name: "".to_string() };
    let _ = fixed_timespan.total_offset();
}
