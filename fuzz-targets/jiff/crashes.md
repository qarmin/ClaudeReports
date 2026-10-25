# jiff - crashes

Source: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = Offset::constant(100i8).to_datetime(BrokenDownTime::parse(vec![127u8,
    127u8 /* x16 */, 127u8],
    include_bytes!("image_0.tiff").to_vec()).unwrap().timestamp().unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/tz/offset.rs:204:21:
invalid time zone offset hours
backtrace:
  &lt;jiff::tz::offset::Offset&gt;::constant (jiff/src/util/constant.rs:20:23)
  repro_jiff_src_tz_offset_rs_204_21_44427e::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::civil::Date as ::core::ops::Sub&lt;jiff::SignedDuration&gt;&gt;::sub(
    Date::ZERO, SignedDuration::from_micros(1000000000000000000i64));</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/civil/date.rs:2182:14:
subing signed duration to date overflowed: number of days derived from duration exceed's Jiff's datetime limits: parameter 'Unix epoch days' is not in the required range of -4371587..=2932896
backtrace:
  &lt;jiff::civil::date::Date as core::ops::arith::Sub&lt;jiff::signed_duration::SignedDuration&gt;&gt;::sub (jiff/src/civil/date.rs:2182:14)
  repro_jiff_src_civil_date_rs_2182_14_59cf21::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::civil::DateTime as ::core::ops::Sub&lt;jiff::SignedDuration&gt;&gt;::sub(
    Offset::ZERO.to_datetime(Timestamp::UNIX_EPOCH),
    SignedDuration::from_micros(-1000000000000000000i64));</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/civil/datetime.rs:2728:14:
subtracting signed duration from datetime overflowed: parameter 'days' is not in the required range of -7304484..=7304484
backtrace:
  &lt;jiff::civil::datetime::DateTime as core::ops::arith::Sub&lt;jiff::signed_duration::SignedDuration&gt;&gt;::sub (jiff/src/civil/datetime.rs:2728:14)
  repro_jiff_src_civil_datetime_rs_2728_14_d19383::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::Span as ::core::ops::Mul&lt;i64&gt;&gt;::mul(
    &lt;jiff::tz::Offset as ::core::ops::Sub&lt;jiff::tz::Offset&gt;&gt;::sub(Offset::UTC,
    Offset::from_seconds(0i32).unwrap()).minutes::&lt;i64&gt;(1i64),
    -1000000000000000000i64);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/span.rs:3049:14:
multiplying `Span` by a scalar overflowed: parameter 'minutes' is not in the required range of -10518456960..=10518456960
backtrace:
  &lt;jiff::span::Span as core::ops::arith::Mul&lt;i64&gt;&gt;::mul (jiff/src/span.rs:3049:14)
  repro_jiff_src_span_rs_3049_14_5ad89a::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::civil::DateTime as ::core::ops::Add&lt;jiff::SignedDuration&gt;&gt;::add(
    DateTime::MIN, Date::MAX.duration_until(Date::ZERO));</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/civil/datetime.rs:2703:14:
adding signed duration to datetime overflowed: failed to add overflowing duration: parameter 'Unix epoch days' is not in the required range of -4371587..=2932896
backtrace:
  &lt;jiff::civil::datetime::DateTime as core::ops::arith::Add&lt;jiff::signed_duration::SignedDuration&gt;&gt;::add (jiff/src/civil/datetime.rs:2703:14)
  repro_jiff_src_civil_datetime_rs_2703_14_ca187e::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::Timestamp as ::core::ops::Add&lt;jiff::SignedDuration&gt;&gt;::add(
    Timestamp::from(Zoned::default()), SignedDuration::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/timestamp.rs:2478:14:
adding signed duration to timestamp overflowed: parameter 'Unix timestamp seconds' is not in the required range of -377705023201..=253402207200
backtrace:
  &lt;jiff::timestamp::Timestamp as core::ops::arith::Add&lt;jiff::signed_duration::SignedDuration&gt;&gt;::add (jiff/src/timestamp.rs:2478:14)
  repro_jiff_src_timestamp_rs_2478_14_420a1b::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::tz::Offset as ::core::ops::Sub&lt;jiff::SignedDuration&gt;&gt;::sub(
    Offset::MAX, SignedDuration::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/tz/offset.rs:1247:31:
subtracting signed duration from offsetsshould not overflow: adding signed duration to time zone offset overflowed: parameter 'time zone offset total seconds' is not in the required range of -93599..=93599
backtrace:
  &lt;jiff::tz::offset::Offset as core::ops::arith::Sub&lt;jiff::signed_duration::SignedDuration&gt;&gt;::sub (jiff/src/tz/offset.rs:1247:31)
  repro_jiff_src_tz_offset_rs_1247_31_ab7062::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::SignedDuration as ::core::ops::Neg&gt;::neg(SignedDuration::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/signed_duration.rs:2608:28:
overflow when negating signed duration
backtrace:
  &lt;jiff::signed_duration::SignedDuration as core::ops::arith::Neg&gt;::neg (jiff/src/signed_duration.rs:2608:28)
  repro_jiff_src_signed_duration_rs_2608_28_f5d9bc::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::tz::Offset as ::core::ops::Add&lt;jiff::Span&gt;&gt;::add(
    Offset::from_seconds(0i32).unwrap(),
    Span::try_from(SignedDuration::from_hours(91i64)).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/tz/offset.rs:1164:14:
adding span to offset should not overflow: parameter 'time zone offset total seconds' is not in the required range of -93599..=93599
backtrace:
  &lt;jiff::tz::offset::Offset as core::ops::arith::Add&lt;jiff::span::Span&gt;&gt;::add (jiff/src/tz/offset.rs:1164:14)
  repro_jiff_src_tz_offset_rs_1164_14_913d56::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::tz::Offset as ::core::ops::Add&lt;jiff::SignedDuration&gt;&gt;::add(
    Offset::from_hours(0i8).unwrap(), SignedDuration::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/tz/offset.rs:1223:14:
adding signed duration to offset should not overflow: adding signed duration to time zone offset overflowed: parameter 'time zone offset total seconds' is not in the required range of -93599..=93599
backtrace:
  &lt;jiff::tz::offset::Offset as core::ops::arith::Add&lt;jiff::signed_duration::SignedDuration&gt;&gt;::add (jiff/src/tz/offset.rs:1223:14)
  repro_jiff_src_tz_offset_rs_1223_14_c395b3::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::SignedDuration as ::core::ops::Mul&lt;i32&gt;&gt;::mul(
    SignedDuration::MIN, 1000000000i32);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/signed_duration.rs:2651:14:
overflow when multiplying signed duration by scalar
backtrace:
  &lt;jiff::signed_duration::SignedDuration as core::ops::arith::Mul&lt;i32&gt;&gt;::mul (jiff/src/signed_duration.rs:2651:14)
  repro_jiff_src_signed_duration_rs_2651_14_65e1f9::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::Timestamp as ::core::ops::Sub&lt;jiff::SignedDuration&gt;&gt;::sub(
    Timestamp::constant(0i64, 0i32), SignedDuration::MAX);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/timestamp.rs:2506:14:
subtracting signed duration from timestamp overflowed: parameter 'Unix timestamp seconds' is not in the required range of -377705023201..=253402207200
backtrace:
  &lt;jiff::timestamp::Timestamp as core::ops::arith::Sub&lt;jiff::signed_duration::SignedDuration&gt;&gt;::sub (jiff/src/timestamp.rs:2506:14)
  repro_jiff_src_timestamp_rs_2506_14_1d0e2b::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let date_time = DateTime::from(DateTime::MIN);
let _ = &lt;jiff::civil::Date as ::core::ops::Add&lt;jiff::SignedDuration&gt;&gt;::add(
    Date::from(date_time), SignedDuration::new(1000000000000000000i64, 0i32));</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/civil/date.rs:2157:14:
adding signed duration to date overflowed: number of days derived from duration exceed's Jiff's datetime limits: parameter 'Unix epoch days' is not in the required range of -4371587..=2932896
backtrace:
  &lt;jiff::civil::date::Date as core::ops::arith::Add&lt;jiff::signed_duration::SignedDuration&gt;&gt;::add (jiff/src/civil/date.rs:2157:14)
  repro_jiff_src_civil_date_rs_2157_14_a430f3::main (src/main.rs:7:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::Zoned as ::core::ops::Sub&lt;jiff::SignedDuration&gt;&gt;::sub(
    Zoned::UNIX_EPOCH, SignedDuration::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/zoned.rs:3786:41:
subtracting signed duration from zoned datetime overflowed: unsigned duration exceeds Jiff's limits: parameter 'signed duration seconds' is not in the required range of -9223372036854775808..=9223372036854775807
backtrace:
  &lt;jiff::zoned::Zoned as core::ops::arith::Sub&lt;jiff::signed_duration::SignedDuration&gt;&gt;::sub (jiff/src/zoned.rs:3786:41)
  repro_jiff_src_zoned_rs_3786_41_2785e2::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::Zoned as ::core::ops::Add&lt;jiff::SignedDuration&gt;&gt;::add(
    Zoned::default(), SignedDuration::from_secs(-1000000000000000000i64));</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/zoned.rs:3743:14:
adding signed duration to zoned datetime overflowed: parameter 'Unix timestamp seconds' is not in the required range of -377705023201..=253402207200
backtrace:
  &lt;jiff::zoned::Zoned as core::ops::arith::Add&lt;jiff::signed_duration::SignedDuration&gt;&gt;::add (jiff/src/zoned.rs:3743:14)
  repro_jiff_src_zoned_rs_3743_14_c602c3::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::SignedDuration as ::core::ops::Div&lt;i32&gt;&gt;::div(
    SignedDuration::from_millis(0i64), 0i32);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/signed_duration.rs:2689:14:
overflow when dividing signed duration by scalar
backtrace:
  &lt;jiff::signed_duration::SignedDuration as core::ops::arith::Div&lt;i32&gt;&gt;::div (jiff/src/signed_duration.rs:2689:14)
  repro_jiff_src_signed_duration_rs_2689_14_0dfb5a::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::civil::Date as ::core::ops::Add&lt;jiff::Span&gt;&gt;::add(
    ISOWeekDate::MAX.date(), Span::new().try_years::&lt;i64&gt;(1i64).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/civil/date.rs:2090:31:
adding span to date overflowed: parameter 'year' is not in the required range of -9999..=9999
backtrace:
  &lt;jiff::civil::date::Date as core::ops::arith::Add&lt;jiff::span::Span&gt;&gt;::add (jiff/src/civil/date.rs:2090:31)
  repro_jiff_src_civil_date_rs_2090_31_0f01b6::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::Timestamp as ::core::ops::Sub&lt;jiff::SignedDuration&gt;&gt;::sub(
    Timestamp::from_second(0i64).unwrap(), SignedDuration::MIN);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/timestamp.rs:2504:14:
signed duration negation resulted in overflow
backtrace:
  &lt;jiff::timestamp::Timestamp as core::ops::arith::Sub&lt;jiff::signed_duration::SignedDuration&gt;&gt;::sub (jiff/src/timestamp.rs:2504:14)
  repro_jiff_src_timestamp_rs_2504_14_d91f77::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::tz::Offset as ::core::ops::Sub&lt;jiff::Span&gt;&gt;::sub(Offset::MAX,
    Offset::MAX.until(Offset::UTC));</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/tz/offset.rs:1187:14:
subtracting span from offsetsshould not overflow: parameter 'time zone offset total seconds' is not in the required range of -93599..=93599
backtrace:
  &lt;jiff::tz::offset::Offset as core::ops::arith::Sub&lt;jiff::span::Span&gt;&gt;::sub (jiff/src/tz/offset.rs:1187:14)
  repro_jiff_src_tz_offset_rs_1187_14_c0aab2::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let span = &lt;jiff::Span as ::core::ops::Neg&gt;::neg(Span::new());
let _ = &lt;jiff::Timestamp as ::core::ops::Sub&lt;jiff::Span&gt;&gt;::sub(Timestamp::MIN,
    span.hours::&lt;i64&gt;(1i64));</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/timestamp.rs:2432:14:
subtracting span from timestamp failed: adding span overflowed timestamp: parameter 'Unix timestamp seconds' is not in the required range of -377705023201..=253402207200
backtrace:
  &lt;jiff::timestamp::Timestamp as core::ops::arith::Sub&lt;jiff::span::Span&gt;&gt;::sub (jiff/src/timestamp.rs:2432:14)
  repro_jiff_src_timestamp_rs_2432_14_00ec22::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let timestamp = Timestamp::constant(0i64, 0i32);
let span = &lt;jiff::civil::DateTime as ::core::ops::Sub&lt;jiff::civil::DateTime&gt;&gt;::
    sub(DateTime::from(Date::default()), DateTime::from_parts(Date::MAX,
    Time::MIN));
let _ = &lt;jiff::Timestamp as ::core::ops::Add&lt;jiff::Span&gt;&gt;::add(timestamp, span);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/timestamp.rs:2403:37:
adding span to timestamp failed: operation can only be performed with units of hours or smaller, but found non-zero 'day' units (operations on `jiff::Timestamp`, `jiff::tz::Offset` and `jiff::civil::Time` don't support calendar units in a `jiff::Span`)
backtrace:
  &lt;jiff::timestamp::Timestamp as core::ops::arith::Add&lt;jiff::span::Span&gt;&gt;::add (jiff/src/timestamp.rs:2403:37)
  repro_jiff_src_timestamp_rs_2403_37_30ae09::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let span = Span::try_from(Date::MAX.duration_until(Date::MIN)).unwrap();
let span2 = span.days::&lt;i64&gt;(0i64);
let span3 = span2.hours::&lt;i64&gt;(0i64);
let _ = &lt;jiff::civil::DateTime as ::core::ops::Add&lt;jiff::Span&gt;&gt;::add(
    DateTime::default(), span3);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/civil/datetime.rs:2630:31:
adding span to datetime overflowed: failed to add overflowing span: parameter 'Unix epoch days' is not in the required range of -4371587..=2932896
backtrace:
  &lt;jiff::civil::datetime::DateTime as core::ops::arith::Add&lt;jiff::span::Span&gt;&gt;::add (jiff/src/civil/datetime.rs:2630:31)
  repro_jiff_src_civil_datetime_rs_2630_31_0c0024::main (src/main.rs:9:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;jiff::civil::DateTime as ::core::ops::Sub&lt;jiff::Span&gt;&gt;::sub(
    DateTime::default(),
    &lt;jiff::Timestamp as ::core::ops::Sub&lt;jiff::Timestamp&gt;&gt;::sub(Timestamp::new(
    0i64, 0i32).unwrap(), Timestamp::MIN));</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/civil/datetime.rs:2655:14:
subtracting span from datetime overflowed: failed to add overflowing span: parameter 'Unix epoch days' is not in the required range of -4371587..=2932896
backtrace:
  &lt;jiff::civil::datetime::DateTime as core::ops::arith::Sub&lt;jiff::span::Span&gt;&gt;::sub (jiff/src/civil/datetime.rs:2655:14)
  repro_jiff_src_civil_datetime_rs_2655_14_eb171c::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let date_time_printer = DateTimePrinter::new();
let _ = date_time_printer.separator(255u8);</pre></td><td valign="top"><pre>thread 'main' panicked at jiff/src/fmt/temporal/printer.rs:160:9:
RFC3339 separator must be ASCII
backtrace:
  &lt;jiff::fmt::temporal::printer::DateTimePrinter&gt;::separator (jiff/src/fmt/temporal/printer.rs:160:9)
  &lt;jiff::fmt::temporal::DateTimePrinter&gt;::separator (jiff/src/fmt/temporal/mod.rs:1262:25)
  repro_jiff_src_fmt_temporal_printer_rs_160_9_fae1d0::main (src/main.rs:5:31)</pre></td></tr>
</table>
