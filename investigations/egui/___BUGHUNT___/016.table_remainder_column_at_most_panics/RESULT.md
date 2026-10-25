# table_remainder_column_at_most_panics

`TableBuilder` sizing in `crates/egui_extras/src/table.rs` (~636-643) feeding `Sizing::to_lengths`. Found by a randomized `TableBuilder` smoke run (random `Column` kinds, `at_least`/`at_most`, resizable, narrow screens, pointer drags) that panicked with `min > max, or either was NaN`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Randomized TableBuilder run | 4000 random tables x 4 frames, debug | 97 panics, all `min > max` from `f32::clamp`, all with a `remainder()` column that has a small `at_most` |
| Single column variants | header "header text" / cell "cell text" in a 400 px screen | `auto().at_most(5)`, `initial(50).at_most(5)`, `remainder().at_most(100)` are fine; `remainder().at_most(5)` panics (min = 9.0 content width, max = 5.0) |

## Status

**Confirmed** - deterministic, 3/3 runs; the panic comes from `f32::clamp`, which asserts in debug and release builds.

**Fully reproducible**: **Yes** (public `TableBuilder`/`Column` API through `Context::run_ui`).

## Class

crash

## Root cause

For a non-resizable, non-clipped column the table asks for at least `max(width_range.min, widest content so far)`. When the content is wider than `width_range.max` this minimum exceeds the column's own maximum, so the `Size` has an inverted range (`min > max`), and `Sizing::to_lengths` calls `range.clamp(..)` for `Size::Remainder`, which panics. Absolute/automatic columns do not clamp there and survive.

## Introduced by

8ef0e85b8 "egui_extras: Improve the auto-sizing of `Table` (#4756)" added `min.max(max_used)`.

## The fix

Cap the content-based minimum at the column's maximum (`patch.diff`, +1/-1). After the patch all four variants run.

- Project test suite (`cargo test -p egui_extras --lib`): 15 passed / 0 failed before -> same after.

## What's not done / open questions

- A user-supplied `at_least` larger than `at_most` (e.g. `auto().at_least(50).at_most(5)`) still panics in the same clamp; that is invalid input.
- `StripBuilder` with `Size::remainder().at_most(..)` goes through the same `Sizing` but has no content-based minimum, so it was not affected in the smoke run.
