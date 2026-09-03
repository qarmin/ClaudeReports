# datetime-range-variant-mismatch-unreachable-safe

`core/src/value/range.rs:397`, inside `AsRange for DicomDateTime`'s `range()` method, has
`match (start, end) { (Naive, Naive) => ..., (TimeZone, TimeZone) => ..., _ => unreachable!() }`
after computing `start = self.earliest()?` and `end = self.latest()?`. Flagged as an unchecked
`unreachable!()` site left over from the prior session's sweep (`projects/dicom-rs.md`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reasoning: read `earliest()` and `latest()` (`core/src/value/range.rs:333-390`) side by side | Static code reading, no build needed | Both methods branch on `self.time_zone()` - a plain accessor on `self` with no interior mutability, called on the exact same `self` reference in both methods, so both must take the same branch (`Some(offset)` -> `PreciseDateTime::TimeZone`, or `None` -> `PreciseDateTime::Naive`) and always agree with each other. Both branches use `?` to propagate any `NaiveDateTime`-construction or ambiguous-local-time (`.single()` returning `None`) failure as `Err` before the ` match (start, end)` in `range()` is ever reached - so a disagreement between `start`'s and `end`'s variant can never occur; either both succeed with matching variants, or one of the two `?`s exits `range()` early |

## Status

**Not reproducible** - ruled out by reading; `start` and `end` are structurally guaranteed to
share the same `PreciseDateTime` variant, since both derive it from the same `self.time_zone()`
call with no possibility of divergence.

## Class

n/a

## Root cause

Not applicable (no bug) - `earliest()`/`latest()` are a genuinely parallel pair keyed on the same
input (`self`), not two independently-maintained match statements that could drift (contrast with
the `FileMetaAttribute` parallel-match-arm bugs from the prior session, candidates 023/024, where
the parallel methods really were independently written and did drift).

## The fix

N/A - no bug found, no fix needed.

## What's not done / open questions

None - the invariant was traced completely (both call sites, both branches, both error paths).
