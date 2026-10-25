# format_with_minimum_decimals_above_six

`emath::format_with_minimum_decimals` at `crates/emath/src/lib.rs:213`. Looked suspicious because it hard-codes `decimals..=6` as the range, which is empty for `decimals > 6` (HEURISTICS: hard-coded bound combined with a caller-controlled value).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Boundary values | `format_with_minimum_decimals(1.5, d)` for d = 5, 6, 7, 8, debug and release | d <= 6 correct; d >= 7 panics in debug (`debug_assert!(min <= max)`), prints only 6 decimals in release |

## Status

**Confirmed** - deterministic pure function, same output on every run (`just test` run once per build mode, plus the manual runs).

**Fully reproducible**: **Yes** (public emath API called directly; there is no second participant).

## Class

logic

## Root cause

The function promises "at least `decimals` decimals" but builds the range `decimals..=6`. For `decimals > 6` the range is inverted: debug builds hit the `min_decimals <= max_decimals` assertion in `format_with_decimals_in_range`, release builds clamp `min_decimals` down to 6 and return fewer decimals than the requested minimum.

## Introduced by

041f2e64b "Move all crates into a `crates` directory (#1940)" is only where `git log -S` stops (a pure move); the code predates it (not determined further).

## The fix

Use `decimals..=decimals.max(6)` (`patch.diff`, +1/-1). After the patch d = 7 and 8 give 7 and 8 decimals in both debug and release.

- Project test suite (`cargo test` in crates/emath): 13 + 19 passed / 0 failed before -> same after.

## What's not done / open questions

- `format_with_decimals_in_range` silently clamps `max_decimals` to 16 in release (and asserts `< 100` in debug); not investigated further.
- Whether maintainers consider decimals > 6 a supported input was not checked; there is no in-repo caller that passes one.
