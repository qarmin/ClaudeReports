# DASH_NEGATIVE_LENGTH_REVERSED_SEGMENTS

`kurbo::dash` / `DashIterator::step` (`kurbo/src/stroke.rs`) never validates that entries in the
caller-supplied `dashes: &[f64]` pattern are non-negative. `dash_iter`'s one-time setup loop
corrects a negative *first* entry while searching for the initial offset position, but
`step()`'s own main loop has no equivalent correction when the running index wraps back onto a
negative slot later on - this looked suspicious enough (same functions/lines already flagged by
sibling candidate 046) to build a standalone repro and confirm it as a distinct, non-crashing
logic bug (wrong/reversed geometry) rather than the NaN/panic class candidate 046 was checking.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: `dash(line_from_(0,0)_to_(100,0), 0.0, &[-5.0, 10.0])` via the real public `kurbo::dash` API, release build | Straight horizontal line, offset `0.0`, dash pattern `[-5.0, 10.0]` | Reproduced: 19 `MoveTo`/`LineTo` pairs emitted, every pair running backwards (`LineTo`'s x < preceding `MoveTo`'s x) instead of forward (+x), matching the candidate description exactly |
| Control comparison with an all-positive pattern `[5.0, 10.0]` on the same line/offset | Same line, same public API | No backwards pairs - confirms the reversal is specific to the negative entry, not a pre-existing property of `dash()` in general |
| Repeat runs (determinism check) | 3 total runs, same release build, same input | 3/3 runs reproduced identically (byte-identical output, confirmed via `md5sum` on the initial harness) |

No debug-assertions build, miri, or fuzzing was needed - the bug is a pure floating-point logic
error (no integer overflow, no `unsafe`), reproducible deterministically on the very first
boundary-value input tried, so escalating past manual construction added nothing.

## Status

**Confirmed**. Reproduces 3/3 runs, release build, `x86_64-unknown-linux-gnu`, no randomness
involved (deterministic floating-point arithmetic on a fixed input) - see `output.txt` for the
actual captured `just test` output (fresh clone -> before -> patch -> after).

**Fully reproducible**: **Yes**. The repro (`repro/src/main.rs`) calls kurbo's real public
`dash()` entry point directly with an ordinary `BezPath`/dash-pattern `&[f64]` - this is a single
external artifact (a dash pattern a real caller, e.g. an SVG `stroke-dasharray` consumer that
doesn't separately reject negative-looking values, could hand straight to `dash()`) with no second
participant being faked; `CONFIRMATION.md` section 11's two-participant bar does not apply here.

## Class

logic (wrong geometry, not a crash/panic/NaN - segments are emitted in reverse point order)

## Root cause

`DashIterator::step()` (`kurbo/src/stroke.rs`, main loop starting at what was line 835 at
`base_commit`) handles a "dash transition" (the `self.dash_remaining < self.seg_remaining`
branch) by:

1. Computing `t1 = seg.inv_arclen(self.dash_remaining, DASH_ACCURACY)` - the parametrization at
   arc-length `self.dash_remaining` along the *remaining* part of the current segment.
2. Advancing `self.dash_ix` (wrapping at `self.dashes.len()`).
3. Re-reading the next dash length raw: `self.dash_remaining = self.dashes[self.dash_ix];` -
   with **no check that this value is non-negative**.

`dash_iter`'s own setup code (a few lines earlier in the same file) *does* guard against a
negative entry - but only once, for the very first `dash_ix`/`dash_remaining` pair, via
`while dash_remaining < 0.0 { dash_ix = (dash_ix + 1) % dashes.len(); dash_remaining +=
dashes[dash_ix]; is_active = !is_active; }`. That loop is never consulted again once `step()`
starts running its own main loop, so every time the wrapping index in step() lands back on the
pattern's negative entry, `self.dash_remaining` is silently set to a negative value again.

On the very next call to `step()`, `self.dash_remaining < self.seg_remaining` is essentially
always true (a negative number is smaller than any non-negative arc length), so the "dash
transition" branch fires immediately and asks `seg.inv_arclen(self.dash_remaining, ...)` -
`inv_arclen`'s binary/ITP search - for the parametrization at a *negative* target arc length. For
a straight `Line`, this returns a `t1` that undershoots the segment's start rather than
overshooting its end, so the emitted sub-segment (`seg.subsegment(0.0..t1)`, or the bare `MoveTo`
in the inactive branch) lands *behind* where the walk currently is instead of ahead of it -
producing the observed backwards `MoveTo`/`LineTo` pairs.

## Introduced by

af22c87 "Add dashing" (2023-05-26). This is the commit that introduced both `dash_iter`'s
setup-time correction loop and `step()`'s own unguarded `self.dash_remaining =
self.dashes[self.dash_ix];` re-read side by side, with the asymmetry between them (one-time
correction vs. never-corrected) present from the very first version of the file. Verified with
`git log -S"dash_remaining = self.dashes[self.dash_ix]"` (only match: a later pure file-move
commit `d67b9dd`) and `git log --follow -p` on `src/stroke.rs`, which shows the exact same
unguarded line already present in `af22c87`'s diff.

## The fix

Mirror `dash_iter`'s own setup-time correction inside `step()`, immediately after re-reading
`self.dash_remaining` from the wrapped index: fold a negative entry into the following one
(advancing the index again, adding the next entry's value, and toggling `is_active`) exactly the
way the initial-offset search already does, repeating until the running remainder is
non-negative.

```rust
self.dash_remaining = self.dashes[self.dash_ix];
while self.dash_remaining < 0.0 {
    self.dash_ix += 1;
    if self.dash_ix == self.dashes.len() {
        self.dash_ix = 0;
    }
    self.dash_remaining += self.dashes[self.dash_ix];
    self.is_active = !self.is_active;
}
```

Verified: after applying this patch, the same `[-5.0, 10.0]` input against the same line no
longer emits any backwards pair - the emitted elements collapse to bare `MoveTo`s only (no
`LineTo`s at all), because the negative entry's fold-in cancels out against the toggle `step()`
already performs at the transition, leaving the walk permanently on the "inactive/gap" side for
this particular (invalid) pattern; there is no reversed geometry left to observe. This matches
`CONFIRMATION.md` section 7's bar: checked against more than the one triggering pattern - a
control run with an all-positive `[5.0, 10.0]` pattern on the same line, offset, and code path
was byte-identical before and after the patch (unaffected), confirming the fix is scoped to the
negative-entry case and does not perturb ordinary, valid input.

Because a negative dash length has no canonical "correct" meaning (it is invalid input, not a
value with a well-defined intended geometry), this fix's goal is narrower than "produce the
intended dash pattern" - it is "never emit backwards/self-contradictory geometry for this input",
which it achieves.

- Project test suite (`cargo test --workspace`, `kurbo/` + `polycool/`): **before** patch - 187
  lib tests passed, 10 (arc-length/offset numerical) tests passed, 55 doctests passed, 1 doctest
  ignored, 0 failed; **after** patch - identical counts (187/10/55 passed, 1 ignored, 0 failed).
  No regressions, no new failures.

## What's not done / open questions

- `dash_offset`-driven negative-length interactions (a negative dash entry combined with a
  nonzero `dash_offset`, or with `stable_dash_order: true`) were not separately swept - the fix
  targets the exact mechanism (the unguarded re-read at the wrap point), which is
  offset/order-agnostic, but a dedicated regression test covering those combinations was not
  added this session.
- No `unsafe`/memory-safety angle exists here (pure `f64` arithmetic, no indexing beyond
  `self.dashes[self.dash_ix]` which is already kept in `0..dashes.len()` by the modulo/wrap
  logic), so miri/ASAN were not applicable and were not run.
- Whether `Stroke`/`StrokeCtx`'s higher-level dash-then-stroke-expansion path (which consumes
  `dash()`'s output) has any additional fallout beyond what `dash()` itself emits (e.g. degenerate
  offset curves from a backwards-then-forwards zig-zag) was not investigated separately - this
  candidate is scoped to `dash()`'s own output.
- No upstream validation was added for negative dash-pattern entries at the public API boundary
  (e.g. rejecting/asserting in `Stroke::with_dashes` or `dash()` itself) - the fix restores
  internal consistency (no reversed geometry) but does not turn "negative dash length" into a
  documented error; a maintainer may prefer input validation over silent folding as the long-term
  fix.
