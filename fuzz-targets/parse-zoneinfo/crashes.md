# parse-zoneinfo - crashes

Source: https://github.com/chronotope/chrono-tz @ 64c3e4daa9fa5a4ee4d5a6098609790142552fe5

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let day_spec = DaySpec::LastOnOrBefore(Weekday::Sunday, 0i8);
let year = Year::Maximum;
let change_time = ChangeTime::UntilDay(year, Month::August, day_spec);
let _ = change_time.year();</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/line.rs:682:18:
internal error: entered unreachable code
backtrace:
  &lt;parse_zoneinfo::line::ChangeTime&gt;::year (parse-zoneinfo/src/line.rs:682:18)
  repro_parse_zoneinfo_src_line_rs_682_18_638450::main (src/main.rs:11:25)</pre></td></tr>
<tr><td valign="top"><pre>let rule_info = RuleInfo { from_year: Year::Minimum, to_year: None,
    month: Month::March, day: DaySpec::FirstOnOrAfter(Weekday::Tuesday, -10i8),
    time: 0i64, time_type: TimeType::Standard, time_to_add: 0i64,
    letters: None };
let _ = rule_info.absolute_datetime(0i64, 0i64, 0i64);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/line.rs:276:18:
why is negative modulus designed so?
backtrace:
  &lt;parse_zoneinfo::line::Weekday&gt;::calculate (parse-zoneinfo/src/line.rs:276:18)
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day::{closure#3} (parse-zoneinfo/src/line.rs:360:47)
  &lt;&amp;mut &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day::{closure#3} as core::ops::function::FnOnce&lt;(i8,)&gt;&gt;::call_once (core/src/ops/function.rs:310:21)
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day (parse-zoneinfo/src/line.rs:375:18)
  &lt;parse_zoneinfo::line::ChangeTime&gt;::to_timestamp (parse-zoneinfo/src/line.rs:644:33)
  &lt;parse_zoneinfo::table::RuleInfo&gt;::absolute_datetime (parse-zoneinfo/src/table.rs:144:20)
  repro_parse_zoneinfo_src_line_rs_276_18_e8ab0b::main (src/main.rs:10:23)</pre></td></tr>
<tr><td valign="top"><pre>let fixed_timespan = FixedTimespan { utc_offset: i64::MAX, dst_offset: 1i64,
    name: "".to_string() };
let _ = fixed_timespan.total_offset();</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/transitions.rs:146:9:
attempt to add with overflow
backtrace:
  &lt;parse_zoneinfo::transitions::FixedTimespan&gt;::total_offset (parse-zoneinfo/src/transitions.rs:146:9)
  repro_parse_zoneinfo_src_transitions_rs_146_9_0d01cb::main (src/main.rs:5:28)</pre></td></tr>
<tr><td valign="top"><pre>let day_spec = DaySpec::Last(Weekday::Saturday);
let _ = day_spec.to_concrete_day(i64::MIN, Month::July);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/line.rs:268:16:
attempt to add with overflow
backtrace:
  &lt;parse_zoneinfo::line::Weekday&gt;::calculate (parse-zoneinfo/src/line.rs:268:16)
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day::{closure#1} (parse-zoneinfo/src/line.rs:335:34)
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day (parse-zoneinfo/src/line.rs:335:22)
  repro_parse_zoneinfo_src_line_rs_268_16_084992::main (src/main.rs:7:22)</pre></td></tr>
<tr><td valign="top"><pre>let change_time = ChangeTime::UntilMonth(Year::Maximum, Month::June);
let _ = change_time.to_timestamp(0i64, 0i64);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/line.rs:672:18:
internal error: entered unreachable code
backtrace:
  &lt;parse_zoneinfo::line::ChangeTime&gt;::to_timestamp (parse-zoneinfo/src/line.rs:672:18)
  repro_parse_zoneinfo_src_line_rs_672_18_37560e::main (src/main.rs:7:25)</pre></td></tr>
<tr><td valign="top"><pre>let rule_info = RuleInfo { from_year: Year::Number(0i64),
    to_year: Some(Year::Minimum), month: Month::April,
    day: DaySpec::Ordinal(0i8), time: 0i64, time_type: TimeType::Standard,
    time_to_add: 0i64, letters: None };
let _ = rule_info.applies_to_year(0i64);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/table.rs:131:18:
internal error: entered unreachable code
backtrace:
  &lt;parse_zoneinfo::table::RuleInfo&gt;::applies_to_year (parse-zoneinfo/src/table.rs:131:18)
  repro_parse_zoneinfo_src_table_rs_131_18_e961da::main (src/main.rs:9:23)</pre></td></tr>
<tr><td valign="top"><pre>let rule_info = RuleInfo { from_year: Year::Maximum, to_year: None,
    month: Month::June, day: DaySpec::Ordinal(0i8), time: 0i64,
    time_type: TimeType::Wall, time_to_add: 0i64, letters: None };
let _ = rule_info.absolute_datetime(0i64, 0i64, i64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/table.rs:144:9:
attempt to subtract with overflow
backtrace:
  &lt;parse_zoneinfo::table::RuleInfo&gt;::absolute_datetime (parse-zoneinfo/src/table.rs:144:9)
  repro_parse_zoneinfo_src_table_rs_144_9_0f55ab::main (src/main.rs:9:23)</pre></td></tr>
<tr><td valign="top"><pre>let format = Format::Placeholder("".to_string());
let _ = format.format_constant();</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/table.rs:297:13:
Expected a constant formatting string
backtrace:
  &lt;parse_zoneinfo::table::Format&gt;::format_constant (parse-zoneinfo/src/table.rs:297:13)
  repro_parse_zoneinfo_src_table_rs_297_13_b0b7de::main (src/main.rs:5:20)</pre></td></tr>
<tr><td valign="top"><pre>let day_spec = DaySpec::LastOnOrBefore(Weekday::Saturday, -100i8);
let rule_info = RuleInfo { from_year: Year::Number(2043728258908512197i64),
    to_year: Some(Year::Number(62i64)), month: Month::February, day: day_spec,
    time: -7833470800563864112i64, time_type: TimeType::Wall,
    time_to_add: i64::MAX, letters: Some("&lt;svg \
    xmlns=\"http://www.w3.org/2000/svg\" width=\"56\" height=\"NaN\" \
    viewBox=\"0 0 1e9 \
    142\"&gt;&lt;defs&gt;&lt;linearG" /* ... 172 more chars */.to_string()) };
let _ = rule_info.absolute_datetime(i64::MAX, 7i64, 4473909183253428745i64);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/line.rs:357:18:
called `Option::unwrap()` on a `None` value
backtrace:
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day (parse-zoneinfo/src/line.rs:357:18)
  &lt;parse_zoneinfo::line::ChangeTime&gt;::to_timestamp (parse-zoneinfo/src/line.rs:644:33)
  &lt;parse_zoneinfo::table::RuleInfo&gt;::absolute_datetime (parse-zoneinfo/src/table.rs:144:20)
  repro_parse_zoneinfo_src_line_rs_357_18_66f125::main (src/main.rs:11:23)</pre></td></tr>
<tr><td valign="top"><pre>let rule_info = RuleInfo { from_year: Year::Maximum, to_year: None,
    month: Month::August, day: DaySpec::Ordinal(0i8), time: 0i64,
    time_type: TimeType::Wall, time_to_add: 0i64, letters: None };
let _ = rule_info.absolute_datetime(0i64, 1i64, i64::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/table.rs:139:31:
attempt to add with overflow
backtrace:
  &lt;parse_zoneinfo::table::RuleInfo&gt;::absolute_datetime (parse-zoneinfo/src/table.rs:139:31)
  repro_parse_zoneinfo_src_table_rs_139_31_5dd042::main (src/main.rs:9:23)</pre></td></tr>
<tr><td valign="top"><pre>let day_spec = DaySpec::LastOnOrBefore(Weekday::Sunday, i8::MAX);
let _ = day_spec.to_concrete_day(0i64, Month::March);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/line.rs:338:59:
attempt to add with overflow
backtrace:
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day (parse-zoneinfo/src/line.rs:338:59)
  repro_parse_zoneinfo_src_line_rs_338_59_1199da::main (src/main.rs:7:22)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;parse_zoneinfo::line::DaySpec as ::core::str::FromStr&gt;::from_str("");</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/line.rs:387:54:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: Empty }
backtrace:
  &lt;parse_zoneinfo::line::DaySpec as core::str::traits::FromStr&gt;::from_str (parse-zoneinfo/src/line.rs:387:54)
  repro_parse_zoneinfo_src_line_rs_387_54_8a2870::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let rule_info = RuleInfo { from_year: Year::Maximum, to_year: None,
    month: Month::October, day: DaySpec::FirstOnOrAfter(Weekday::Tuesday, 0i8),
    time: 0i64, time_type: TimeType::Standard, time_to_add: 0i64,
    letters: None };
let _ = rule_info.absolute_datetime(1000000000000000000i64, 0i64, 0i64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  parse_zoneinfo::line::{impl#13}::to_timestamp::seconds_until_start_of_year (src/line.rs:580)
  parse_zoneinfo::line::{impl#13}::to_timestamp::time_to_timestamp (src/line.rs:622)
  parse_zoneinfo::line::ChangeTime::to_timestamp (src/line.rs:645)
  parse_zoneinfo::table::RuleInfo::absolute_datetime (src/table.rs:144)
  repro_timeout_in_parse_zoneinfo_table_ruleinfo_absolute_datetime_715975::main (src/main.rs:10)
  (5 std/libc frames omitted)</pre></td></tr>
<tr><td valign="top"><pre>let day_spec = DaySpec::FirstOnOrAfter(Weekday::Friday, 99i8);
let _ = day_spec.to_concrete_day(0i64, Month::December);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/line.rs:366:50:
called `Result::unwrap()` on an `Err` value: "Cannot wrap year from dec-&gt;jan"
backtrace:
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day::{closure#3} (parse-zoneinfo/src/line.rs:366:50)
  &lt;&amp;mut &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day::{closure#3} as core::ops::function::FnOnce&lt;(i8,)&gt;&gt;::call_once (core/src/ops/function.rs:310:21)
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day (parse-zoneinfo/src/line.rs:375:18)
  repro_parse_zoneinfo_src_line_rs_366_50_55cb0b::main (src/main.rs:7:22)</pre></td></tr>
<tr><td valign="top"><pre>let day_spec = DaySpec::Last(Weekday::Tuesday);
let _ = day_spec.to_concrete_day(i64::MIN, Month::January);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/line.rs:265:28:
attempt to subtract with overflow
backtrace:
  &lt;parse_zoneinfo::line::Weekday&gt;::calculate (parse-zoneinfo/src/line.rs:265:28)
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day::{closure#1} (parse-zoneinfo/src/line.rs:335:34)
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day (parse-zoneinfo/src/line.rs:335:22)
  repro_parse_zoneinfo_src_line_rs_265_28_06cf12::main (src/main.rs:7:22)</pre></td></tr>
<tr><td valign="top"><pre>let month = Month::May;
let day_spec = DaySpec::FirstOnOrAfter(Weekday::Friday, i8::MAX);
let _ = day_spec.to_concrete_day(0i64, month);</pre></td><td valign="top"><pre>thread 'main' panicked at parse-zoneinfo/src/line.rs:358:60:
attempt to add with overflow
backtrace:
  &lt;parse_zoneinfo::line::DaySpec&gt;::to_concrete_day (parse-zoneinfo/src/line.rs:358:60)
  repro_parse_zoneinfo_src_line_rs_358_60_5d700e::main (src/main.rs:8:22)</pre></td></tr>
<tr><td valign="top"><pre>let change_time = ChangeTime::UntilMonth(Year::Number(1000000000000000000i64),
    Month::July);
let _ = change_time.to_timestamp(0i64, 0i64);</pre></td><td valign="top"><pre>timeout: still running after 9s, interrupted here
backtrace:
  parse_zoneinfo::line::{impl#13}::to_timestamp::seconds_until_start_of_year (src/line.rs:580)
  parse_zoneinfo::line::{impl#13}::to_timestamp::time_to_timestamp (src/line.rs:622)
  parse_zoneinfo::line::ChangeTime::to_timestamp (src/line.rs:641)
  repro_timeout_in_parse_zoneinfo_line_changetime_to_timestamp_f7b12c::main (src/main.rs:7)
  (8 std/libc frames omitted)</pre></td></tr>
</table>
