# core-datetime-partial-parsing-arithmetic-safe

`core/src/value/deserialize.rs` (parsing DICOM DA/TM/DT wire strings into `DicomDate`/`DicomTime`/
`DicomDateTime`) and `core/src/value/partial.rs` (the partial-precision date/time types
themselves, including their construction and `NaiveDate`/`NaiveTime` conversions) were flagged in
`IDEAS.md`'s backlog as an unchecked area matching HEURISTICS.md's "arithmetic on sizes/lengths
derived from input" and "boundary value" patterns (leap years, month/day ranges, fractional-second
precision, leap-second correction) - untouched by either bug-hunt session before this one (the
`AsRange` trait built on top of these types was checked as candidate `029`, but the underlying
parsing/construction was not).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reading of every subtraction/exponentiation/narrowing-cast in both files, tracing each back to its guard | Static code reading, no build needed | Every arithmetic site found is guarded before use: `read_number_unchecked`'s `buf[0] - b'0'` (deserialize.rs:324) is only reachable through the public `read_number`, which validates every byte is an ASCII digit first; `u8::try_from(n).unwrap()` (deserialize.rs:167) has `n` clamped via `usize::min(6, max)` beforehand, always in `0..=6`; the leap-second correction `microsecond - 1_000_000` (partial.rs:667/848/891, three call sites) is guarded by `if microsecond >= 1_000_000 && second == 59` immediately before each; `6 - frac_precision as u32` (partial.rs:627) is guarded by `if !(1..=6).contains(&frac_precision) { return Err(...) }` a few lines above, and the only caller (`deserialize.rs`'s `parse_time_partial`) can only reach `from_hmsf` with `frac_precision` (`fp`) in that same range, since `fp = 0` implies `n = 0`, which makes the preceding `read_number(&buf[0..0])` call return `Err` (empty-text rejection) and bail out via `?` before `from_hmsf` is ever called |
| Checked the four `unreachable!("fp outside expected range 0..=6")` sites (partial.rs:486, 514, 545, 596) for reachability, per the project's own established "grep unreachable!() and check each site" technique | Static code reading | All four match on a private `DicomTimeImpl::Fraction(.., fp)` tuple's `fp` field, constructed only via the same `from_hmsf` (validated 1..=6) or `try_from(&NaiveTime)` (which computes a valid `frac_precision` internally, not from external input) - no public constructor bypasses the range check, so these are safe by the same "closed private type, guarded constructor" pattern already established for other candidates this session (`028`, `029`, `030`) |

## Status

**Not reproducible** - ruled out by tracing every arithmetic site to an upstream guard; no gap
found across either file's date/time parsing and construction logic.

## Class

n/a

## Root cause

Not applicable (no bug) - this is a mature, evidently carefully-guarded part of the codebase (every
narrow-integer operation found already has an explicit range check or validated caller immediately
upstream). Worth noting for a future session: this area's *robustness* here contrasts with some of
this session's other findings (`027`, `033`, `034`) in less centrally-tested corners of the
codebase (JSON deserialization, network PDU encoding) - suggesting date/time parsing has received
more attention/testing historically (consistent with `core/src/value/deserialize.rs`'s own
extensive inline unit test coverage, visibly denser than some other files checked this session).

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

- `DicomDate`'s equivalent construction/validation logic (`from_ymd`/`from_ym`/`from_y`,
  `check_component`, leap-year handling) was read but not exhaustively re-derived arithmetic-site-
  by-arithmetic-site to the same depth as `DicomTime`'s - no suspicious pattern was noticed, but
  this is a lighter-touch check than the time-parsing side got.
- `DicomDateTime`'s own construction (combining a `DicomDate` and optional `DicomTime` plus an
  optional UTC offset) was not separately re-checked here beyond what candidate `029` already
  covered for its `AsRange` implementation specifically.
- Differential testing against `chrono`'s own date/time validation (e.g. fuzzing
  `DicomDate::from_ymd`/`DicomTime::from_hms_micro` against arbitrary `u16`/`u8`/`u32` inputs and
  comparing against `NaiveDate`/`NaiveTime`'s own `_opt` constructors) was not attempted - manual
  reasoning was judged sufficient given how consistently guarded every site already was, but a
  short property-based/fuzz pass here would be cheap and could still be worthwhile in a future
  session with time to spare.
