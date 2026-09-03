# core-dicomtime-leap-second-earliest-latest

`core/src/value/partial.rs`'s own `check_component` explicitly allows `DateComponent::Second` in
range `0..=60` - DICOM's TM (Time) value representation permits second=60 to represent a leap
second (PS3.5). But `DicomTime`'s `AsRange::earliest()` and `AsRange::latest()`
(`core/src/value/range.rs`) pass that second value straight into `chrono::NaiveTime::
from_hms_micro_opt`, which does **not** accept second=60 (chrono represents leap seconds
differently - as second=59 with an extra microsecond offset in the `1_000_000..2_000_000`
range) and returns `None`, which both methods then turn into `Err(InvalidTimeMicro)`.

The reverse conversion (`impl TryFrom<&NaiveTime> for DicomTime`, elsewhere in this same file
already handles this exact translation correctly - `earliest()`/`latest()` are the ones missing
the equivalent adjustment in the forward direction.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-065): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/dicom-time-leap-second` (commit `15b736a5`) fixing this exact gap | Confirmed unmerged |
| Confirmed `DateComponent::Second`'s valid range really does include 60 by reading `core/src/value/partial.rs`'s `check_component`, and that `DicomTime::from_hms(23, 59, 60)` (a fully public constructor) succeeds | Static code reading | `0..=60` confirmed; `from_hms` doesn't reject 60 |
| Temporary in-tree `#[test]` in `core/src/value/range.rs` (reverted before write-up): construct `DicomTime::from_hms(23, 59, 60)`, call `.earliest()` and `.latest()` | `cargo test -p dicom-core leap_second_earliest_latest_repro` | Both calls return `Err(InvalidTimeMicro { h: 23, m: 59, s: 60, f: 0, .. })` for what is, per this crate's own validation rules, a legitimately constructible value |
| Standalone `repro/` (path-dependency on a pristine clone of `df2b9e42`, via `just test`) | `cargo run` | Reproduces the same failure deterministically; see `backtrace.txt` |

## Status

**Confirmed** - deterministic, reproduces every run, no fixed seed needed (a fixed, valid input
per this crate's own component-range validation).

## Class

`logic` (incorrect `Err` result - not a crash - for a value the crate's own validation
explicitly accepts as legal; `AsRange::earliest`/`latest` are used, among other things, wherever
a `DicomTime`/`DicomDateTime` needs to be compared, ranged, or converted to a precise instant,
e.g. range/comparison queries in C-FIND, or any application code calling these public methods
directly).

## Root cause

`chrono`'s `NaiveTime::from_hms_micro_opt` doesn't model leap seconds the way DICOM's TM value
representation does - it represents a leap second as `second=59` with a microsecond value
offset into `1_000_000..2_000_000`, not as a literal `second=60`. The reverse conversion in this
same file (`TryFrom<&NaiveTime> for DicomTime`) already performs this translation; `earliest()`
and `latest()` simply never had the equivalent forward-direction adjustment applied when they
were written.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Apply the same translation used by the reverse conversion, in both `earliest()` and `latest()`,
immediately before calling into chrono:

```rust
// chrono encodes leap seconds as second=59, microsecond in 1_000_000..2_000_000
let (s, f) = if *s == 60 { (59u8, f + 1_000_000) } else { (*s, f) };
```

See `patch.diff` for the exact diff (scoped to `core/src/value/range.rs` only, both call sites).

- `cargo build -p dicom-core`: builds cleanly with the fix applied.
- `cargo test -p dicom-core --lib` (against the patched pristine clone): **68 passed / 0
  failed** - unchanged pass count from before the fix (minus the temporary repro test itself,
  removed before finalizing), confirming zero regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): fails (panics via `.expect()` on the returned `Err`) before the patch;
  both `earliest()` and `latest()` succeed after.
