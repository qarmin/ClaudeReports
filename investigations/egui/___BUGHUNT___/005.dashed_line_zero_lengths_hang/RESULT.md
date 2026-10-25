# dashed_line_zero_lengths_hang

`dashes_from_line` in `crates/epaint/src/shapes/shape.rs:553`, reached from the public `Shape::dashed_line`, `dashed_line_with_offset`, `dashed_line_many[_with_offset]`. Looked suspicious because the `while position_on_segment < segment_length` loop only terminates if the dash/gap lengths advance the position (HEURISTICS: loop whose progress depends on caller-supplied step sizes), and it indexes `dash_lengths[step]` without checking the slices are non-empty.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | loop termination and indexing in `dashes_from_line` | no progress guard; no emptiness guard |
| Standalone repro (public API) | 100 unit long path, debug build, `ulimit -v 6000000`, each case in a child process with a 5 s watchdog | `dashed_line(.., 5.0, 5.0)` returns 10 shapes; `dashed_line(.., 0.0, 0.0)` never returns and grows memory until the 6 GB address-space limit aborts it (SIGABRT, in under 5 s); `dashed_line_with_offset(.., &[], &[], 0.0)` panics with index out of bounds (exit 101) |

## Status

**Confirmed** - 3/3 runs of `just test`-style cycle plus 2 manual runs, debug build, deterministic.

**Fully reproducible**: **Yes** (public `Shape` API, no second participant; a memory limit is used only so the repro does not exhaust the machine).

## Class

crash

## Root cause

With `dash_length == 0 && gap_length == 0` the position never advances, so the `while` loop pushes a zero-length `Shape::line_segment` per iteration forever (unbounded memory growth, a hang with no limit). With empty `dash_lengths`/`gap_lengths` slices (equal lengths, so the existing `assert_eq!` passes) `dash_lengths[step]` panics.

## Introduced by

fc18d6f8f "Add support for dashed lines with offset (#3720)" introduced the slice-based API and the empty-slice index panic. The zero-length hang exists in the older single-length implementation too (not determined further).

## The fix

Return early (no dashes) when the sum of all dash and gap lengths is not a positive finite number (`patch.diff`, +5/-0). After the patch the `zero` and `empty` cases return 0 shapes immediately and the control case is unchanged.

- Project test suite (`cargo test -p epaint --lib --features default_fonts`): 95 passed / 0 failed before -> 95 passed / 0 failed after.

## What's not done / open questions

- Extremely small but positive lengths (e.g. 1e-30 on a long path) can still stall because adding to a large `f32` position rounds to no change; not tried.
- A maintainer may prefer drawing a solid line, or an assert, instead of drawing nothing for a zero pattern.
- A negative `dash_offset` starts the first dash before the path start (extrapolated); not judged here.
