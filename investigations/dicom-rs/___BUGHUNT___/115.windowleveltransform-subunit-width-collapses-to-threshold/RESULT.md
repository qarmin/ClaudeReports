# windowleveltransform-subunit-width-collapses-to-threshold

`pixeldata/src/transform.rs`'s `WindowLevelTransform::new`/`linear` looked suspicious after
reading upstream PR `Enet4/dicom-rs#826` (open, unmerged as of `df2b9e42`) fetched via this
session's GitHub PR raw-sync - it reports real devices (Agfa CR, per the PR body) emitting a
`WindowWidth` below `1.0` once combined with a small `RescaleSlope`, and describes an incorrect
rendered image (collapsed to black/white) as a result.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading of `WindowLevelTransform::new`/`window_level_linear` | n/a | Found the exact mechanism: `new()` clamps any `width < 1` up to `1.0` before storing it, and `window_level_linear`'s own formula divides by `(ww - 1)`, using `min = wc - 0.5 - (ww-1)/2` / `max = wc - 0.5 + (ww-1)/2` - at `ww == 1` these collapse to the same point, turning the windowing into a single-point hard threshold |
| Standalone repro calling the public `WindowLevelTransform::linear`/`apply` API with the real-world tag values quoted in PR #826 (`WindowCenter = 0.4565323565`, `WindowWidth = 0.9130647131`) across a range of input values | release build, deterministic (pure function, no I/O/randomness) | Reproduced: every sample strictly inside the declared window (`0.1` through `0.9`) mapped to exactly `255` (only the two boundary/outside samples, `-0.05` and `1.0`, differ), i.e. one bit of information instead of a gradient |

No further tools were needed - this is a pure, deterministic arithmetic function with no
adversarial-input/parsing surface, so a single code-reading pass plus a direct repro was
sufficient (soft cap of 3 not needed).

## Status

**Confirmed** - deterministic, pure-function bug: reproduces on every run (not sampled/flaky;
there is no randomness or I/O in the code path at all, so a repeat-count doesn't apply the way it
would for a fuzz-found crash).

**Fully reproducible**: **Yes**. This is not a "one participant reacts to another's wire input"
finding (see `CONFIRMATION.md` section 11) - it's a self-contained arithmetic defect in a public
library function, reproduced by calling that exact function (`WindowLevelTransform::linear(..).apply(..)`,
the same public API `pixeldata`'s own `build_monochrome_image`/`to_dynamic_image` call internally)
with real-world-sourced trigger values quoted verbatim from an actual reported file in PR #826's
description, not synthesized to merely look plausible.

## Class

logic

## Root cause

PS3.3 C.11.2.1.2.1 defines the `LINEAR` VOI LUT function in terms of `window_width - 1` in its
denominator/half-width term. `WindowLevelTransform::new` (and its `linear()` shorthand) clamped
any width below `1.0` up to exactly `1.0` "for safety" - but at `width == 1.0`, `window_level_linear`'s
own `min`/`max` formulas (`wc - 0.5 - (ww-1)/2` and `wc - 0.5 + (ww-1)/2`) both reduce to `wc - 0.5`
with the `(ww-1)` term zeroed out, so `min == max`. The function's own branching (`value <= min ->
0`, `value > max -> y_max`, else a linear interpolation) then has an empty "else" range - every
input value falls into one of the two boundary branches, producing a hard binary threshold instead
of the intended linear ramp. This isn't a rare edge case for real files: any modality whose
`RescaleSlope` maps stored pixel values onto a small floating-point range (the PS3.3 standard
imposes no lower bound on `WindowWidth`, and normalizing rescales naturally produce sub-1.0
windows) hits this immediately, silently turning a real gradient image into a black/white one with
no error, warning, or panic - a genuinely wrong result, not a crash.

## Introduced by

9ff19ee3 "Implement LUT" (the original clamp-to-1 implementation, effectively present since
Window/Level LUT support was first added to this crate).

## The fix

Changed `WindowLevelTransform::new` so that requesting the `LINEAR` function with a width `< 1.0`
transparently switches the transform to `LINEAR_EXACT` instead (clamping only the width to `>= 0.0`,
matching `LINEAR_EXACT`'s own already-correct clamp) rather than clamping the width itself up to
`1.0`. `LINEAR_EXACT`'s formula (PS3.3 C.11.2.1.3.2, `min = wc - ww/2`, `max = wc + ww/2`) has no
`(ww - 1)` term and stays non-degenerate for any `width >= 0`, so it correctly reproduces a smooth
gradient for the same sub-unit width instead of collapsing to a threshold. `width == 1.0` (the
threshold case explicitly described by the standard) is left untouched - only `width < 1.0` is
redirected. The `Sigmoid` branch is unaffected (still clamped to `>= 1.0`; its own formula has no
comparable degeneracy at `ww == 1`, and PS3.3 doesn't define a width-independent alternative for it).

Verified against the repro: before the fix, all 6 interior samples of the real-world window
(`0.1`..`0.9`) mapped to the single value `255.0` (`distinct interior output values: 1`); after the
fix, the same 6 samples produced 6 distinct, monotonically increasing values spanning `~28` to
`~251`, with the window center (`0.4565323565`) landing at `127.5` (exact midpoint) - see
`output.txt` for both full runs.

- Project test suite: `cargo test -p dicom-pixeldata` - 39 passed, 0 failed, 2 ignored before AND
  after (identical counts, zero regressions). Full workspace `cargo test --workspace --exclude
  dcms` (dcms excluded: untracked scratch crate belonging to a different session, not part of this
  workspace's tracked tree) - 0 failures anywhere before or after.

## What's not done / open questions

- Upstream PR `Enet4/dicom-rs#826` independently arrived at the same fix (confirmed by reading its
  diff during discovery) - this candidate's fix was derived and verified independently from first
  principles (reading the standard's own formulas), not copied, but the resulting code is
  substantively the same shape. If that PR is merged upstream first, this candidate becomes a
  duplicate-confirms-independently case, not a new finding.
- Not checked this session: whether `VoiLutTransform` (the explicit-LUT-table sibling transform in
  the same file, distinct from `WindowLevelTransform`) has any comparable degenerate-denominator
  case for a narrow/zero-width table - it uses a fundamentally different mechanism (direct table
  lookup with a bit-shift, no division by width), so this specific bug class likely doesn't apply,
  but this wasn't independently verified.
- Not checked: whether any caller (`pixeldata/src/lib.rs`'s `build_monochrome_image`) constructs a
  `WindowLevelTransform` from a `WindowCenter`/`WindowWidth` pair sourced from a *multi-valued*
  DICOM attribute (`VM > 1`, one window per frame) - only the single-value construction path was
  exercised here.
