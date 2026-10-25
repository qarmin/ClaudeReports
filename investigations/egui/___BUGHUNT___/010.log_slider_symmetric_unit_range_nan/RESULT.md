# log_slider_symmetric_unit_range_nan

`logarithmic_zero_cutoff` in `crates/egui/src/widgets/slider_core.rs:716`, used by every logarithmic `Slider` whose range crosses zero. Found by a headless slider fuzz (random ranges, values, flags, real pointer events) whose debug build reported `Bad cutoff NaN for min -1.0 and max 1.0`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Randomized headless slider run | 4000 random Slider/DragValue configs x 4 frames with pointer press/drag, debug build | panic message `Bad cutoff NaN for min -1.0 and max 1.0` (6 hits) plus the documented `You should use a logarithmic range` assertion for infinite non-log ranges (not a bug) |
| Grid | 16 x 16 range bounds (+-inf, +-1e30, +-100, +-10, +-1, +-0.1, +-1e-6, 0), 3 start values, f32 and f64, press and 3 drags | the only failing range is -1.0..=1.0 (12 of 1536 combinations, i.e. every start value and type) |
| Standalone repro | `Slider::new(&mut value, -1.0..=1.0).logarithmic(true)`, press on the handle and drag to 25% and 90% of the rail; -10..=10 as control | debug: panic. Release: the slider gets stuck at -1.0 (value per frame 0.5, 0.5, -1.0, -1.0, -1.0) while the control moves normally (0.0, -0.01, 2.0) |

## Status

**Confirmed** - reproduces 3/3 runs in debug and release (deterministic).

**Fully reproducible**: **Yes** (real `Slider` widget driven by real pointer events through `Context::run_ui`).

## Class

logic

## Root cause

For a logarithmic slider that crosses zero the position of zero on the rail is `min_magnitude / (min_magnitude + max_magnitude)` with magnitudes `|log10(|min|)|` and `|log10(max)|`. For `-1.0..=1.0` both are `log10(1) = 0`, so the cutoff is `0.0 / 0.0 = NaN`. Debug builds hit the `0.0 <= cutoff <= 1.0` assertion right after; release builds carry the NaN into `value_from_normalized`/`normalized_from_value`, so the handle position and the value it maps to are wrong (the slider jumps to -1.0 and cannot be moved).

## Introduced by

33aa4d698 "Fix typos (#2866)" is only where `git log -S` stops (a mass rename); the function is older (not determined further).

## The fix

If both magnitudes are zero use a cutoff of 0.5, the symmetric answer (`patch.diff`, +7/-1). After the patch the repro neither panics nor sticks: -1..=1 gives 0.5, 0.0, 0.0, -0.002, 0.3 over the same frames.

- Project test suite (`cargo test -p egui --lib`): 86 passed / 0 failed before -> 86 passed / 0 failed after.

## What's not done / open questions

- A range like -1.0..=100.0 gives a cutoff of exactly 0.0 (no room for the negative part); that is the same formula but was not examined.
- The same formula in a second slider copy (if any) was not searched for.
- The random run also reported `from.start() and from.end() should not be equal` (debug assert in `emath::remap`) for an `f32` slider with range `-inf..=1.0`, logarithmic, `step_by(1e-9)`; it did not reproduce in the plain grid and is left in `IDEAS.md`.
