# sizing_remainder_min_overflows_available

`Sizing::to_lengths` in `crates/egui_extras/src/sizing.rs:118`, used by `StripBuilder`, `TableBuilder` and every `Size::remainder().at_least(..)` layout. Looked suspicious because the "remainders that cannot shrink to the average" step decides which remainders sit at their minimum using the average computed before any of them were taken out (HEURISTICS: a value computed once but invalidated by the loop that follows it).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `avg_remainder_length` is computed once, then the loop subtracts each minimum that is above it | taking one remainder out lowers the real average, so another remainder can be under its minimum without being detected |
| Hand-built strips | horizontal `StripBuilder`, spacing 0, 30 points, remainders [min 11, min 10, none] | cells 11, 10, 9.5 = 30.5 points in 30 available (and 90.5 in 90 for minimums 31, 30) |
| Randomized | 3000 random remainder-only strips (2-6 cells, minimums 0-39, 20-169 points), release | 286 (9.5%) exceed max(available, sum of minimums), worst excess 11 points |

## Status

**Confirmed** - deterministic (fixed inputs, fixed seed), 3/3 runs, debug and release.

**Fully reproducible**: **Yes** (real `StripBuilder` laid out in a headless `Context`).

## Class

logic

## Root cause

`to_lengths` first counts the space left for remainders, computes `avg = floor(left / n)` once and then, for every remainder with `min > avg`, subtracts its minimum from the pool and decrements `n`. The new average is lower than `avg`, so a remainder whose minimum is between the new and the old average is not recognised as pinned; the final `range.clamp(new_avg)` then raises it to its minimum without taking that space out of the others, and the cells together are wider than the space they are laid out in (a strip or table then overflows its parent).

## Introduced by

8cdffc4e2 "Replace uses of `RangeInclusive<f32>` with `emath::Rangef` (#3221)" is the oldest commit that `git log -S` finds for the `avg_remainder_length < range.min` line; the earlier form of the logic was not followed (not determined further).

## The fix

Repeat the pin-minimums step until no more remainders need pinning (`patch.diff`, +19/-7). After the patch the hand-built strips give 11 + 10 + 9 = 30 and 31 + 30 + 29 = 90, and the randomized run reports 0 of 3000 over.

- Project test suite (`cargo test -p egui_extras --lib`): 15 passed / 0 failed before -> 15 passed / 0 failed after (includes the existing `test_sizing`).

## What's not done / open questions

- Only remainder-only strips were fuzzed; strips mixing absolute/relative sizes with remainders use the same code but were not run in the random check.
- Whether `Size::Absolute` sizes that exceed the space should shrink was not looked at.
