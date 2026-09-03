# RECT_INSCRIBED_RECT_NEGATIVE_ASPECT_RATIO_INVERTED

`kurbo/src/rect.rs:581-608` `Rect::inscribed_rect_with_aspect_ratio` computes `new_width`/
`new_height` directly from a signed `aspect_ratio`; for a negative `aspect_ratio` (not documented
as forbidden, not panicking, only defined in the doc comment as "specified fractionally, as
`width / height`") this makes the size term negative, so `gap` exceeds half the original
width/height, and the returned `Rect` has `x0 > x1` (or `y0 > y1`) - an inverted-coordinate result
that no longer satisfies the doc's stated invariant ("the largest possible `Rect` ... fully
contained in `self`") in the ordinary min/max sense.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (public API, real end-to-end call, no faked participant) | `Rect::new(0,0,10,10).inscribed_rect_with_aspect_ratio(-1.0)` (square outer, "shrink x" branch) and two more inputs in the same class ("shrink x" on a non-square outer with `aspect_ratio = -0.5`, "shrink y" branch with `aspect_ratio = -4.0`), `cargo run --release` | All three negative-ratio inputs produce a `Rect` with `x0 > x1` or `y0 > y1` (e.g. `Rect { x0: 10.0, y0: 0.0, x1: 0.0, y1: 10.0 }`, `width() == -10`) - reproduces 3/3 release runs plus 1 debug run, byte-identical each time (pure `f64` arithmetic, no randomness) |
| Reading the existing test suite (`kurbo/src/rect.rs`'s own `#[test] fn contained_rect_with_aspect_ratio` / `fn inscribed_rect_with_aspect_ratio`) | `cargo test -p kurbo --lib rect::tests` on pristine code | Both test functions already contain a `// negative aspect ratio` case (`aspect_ratio = -1.0`) that hard-codes the *inverted* result (`[0.0, 15.0, 10.0, 5.0]`, i.e. `y0=15 > y1=5`) as the asserted `expected` value - so this exact "bug" is currently locked in by the project's own test suite as intentional/expected behavior for a negative ratio, not merely an unnoticed edge case. This is an important qualifier on the finding (see "Root cause" and "What's not done" below). |
| Applying a bounded fix (use `aspect_ratio.abs()` for the size terms) and re-running the repro + full test suite, before and after | `cargo test -p kurbo --lib` and `cargo test --workspace` (debug build, default toolchain) | With only the production-code lines changed, 2 of the 187 existing tests fail (the two hard-coded negative-ratio cases described above) - confirms the sanity-check angle from candidate 031 doesn't catch this (the resulting rect's `size().width`/`size().height` magnitudes are unchanged by the coordinate swap, so a magnitude-only sanity check passes either way); updating those 2 test cases' expected values to the corrected (non-inverted) coordinates alongside the production fix brings the suite back to 187/187 passing |

3 approaches were used, the second doubling as extra confirmation and as the source of the
important "existing tests already encode this" qualifier - no further escalation needed (this is a
pure boolean/arithmetic-sign defect on `f64`, not an adversarial-input class needing miri/ASAN/
fuzzing).

## Status

**Confirmed** - deterministic, reproduces 3/3 release runs + 1 debug run (pure `f64` arithmetic, no
randomness, no build-profile dependence), against kurbo's real public `Rect::inscribed_rect_with_aspect_ratio`
API called directly with the one negative `f64` argument that triggers it.

**Fully reproducible**: **Yes**. This finding is not about how one participant reacts to input
produced by another participant (no peer/caller/wire relationship) - it's a single, real, public
API call (`outer.inscribed_rect_with_aspect_ratio(-1.0)`) producing a result that violates the
method's own documented invariant. `CONFIRMATION.md` section 11's "faked second participant"
concern does not apply; `repro/` calls the real function end-to-end with the real public API.

## Class

logic

## Root cause

In the "shrink x" branch (`self_aspect.abs() >= aspect_ratio.abs()`):
```rust
let new_width = height * aspect_ratio;             // negative when aspect_ratio < 0
let gap = (width - new_width) * 0.5;               // > width/2 when new_width < 0
let x0 = self.x0 + gap;
let x1 = self.x1 - gap;
Rect::new(x0, self.y0, x1, self.y1)                // x0 > x1 when gap > width/2
```
and symmetrically in the "shrink y" branch with `new_height = width * aspect_ratio.recip()`. Only
the *magnitude* of `aspect_ratio` should determine how much the outer rect shrinks along the
relevant axis (the branch-selection conditions already compare `.abs()` values on both sides), but
the size computation itself uses the signed value, so a negative `aspect_ratio` produces a
negative `new_width`/`new_height`, which pushes `gap` past half the original extent and swaps
`x0`/`x1` (or `y0`/`y1`) in the result.

Notably, `kurbo`'s own existing test suite already has a `// negative aspect ratio` case for
exactly this input (`aspect_ratio = -1.0` on a `10x20` outer) and hard-codes the resulting
*inverted* rect (`[0.0, 15.0, 10.0, 5.0]`) as the expected value - i.e., this behavior was observed
and written into the tests at some point, without the coordinate inversion itself being flagged.
`Rect::width()`'s own doc comment ("Note: nothing forbids negative width") shows the type
in general tolerates negative-width representations elsewhere in the API, which is likely why this
was not previously treated as an error - but for *this specific method*, the doc's own promise
("the largest possible `Rect` ... fully contained in `self`") is not met literally by an inverted
result: e.g. calling `.contains(point)` on the inverted output would return `false` for every point
in the rectangle's own interior, since `contains` checks `x >= x0 && x < x1` and no `x` satisfies
that when `x0 > x1`.

## Introduced by

bcae24f0 "Add `contained_rect_with_aspect_ratio` fn" (2020-05-14) - `git log -S"contained_rect_with_aspect_ratio"`
shows this is the commit that introduced the function from scratch; `git show bcae24f0:src/rect.rs`
confirms the original code already computed the "else" branch's size term (there, `new_height =
width * aspect_ratio`, since the original convention was `aspect_ratio = height / width`) directly
from the signed `aspect_ratio`, with the same consequence for a negative input (an inverted rect on
the axis handled by that branch). The later refactor f90c15a "Add `aspect_ratio_width`... deprecate
`aspect_ratio`" (2025-08-22, #486) renamed the function, flipped the aspect-ratio convention
(`width/height` instead of `height/width`), and in doing so moved which branch/axis the bug affects
(from the height axis in the original code to the width axis in the "shrink x" branch of the
current code) and additionally introduced the hard-coded-inverted-result test case described above
- but the underlying sign-handling defect itself predates that commit and has existed since the
function was first added in 2020.

## The fix

Use `aspect_ratio.abs()` instead of the signed `aspect_ratio` in both size computations:
`new_height = width * aspect_ratio.abs().recip()` (was `width * aspect_ratio.recip()`) and
`new_width = height * aspect_ratio.abs()` (was `height * aspect_ratio`). This guarantees
`new_width`/`new_height` are non-negative (given non-negative `width`/`height` and non-zero
`aspect_ratio`), which keeps `gap <= width/2` (resp. `height/2`), so the returned `Rect` always has
`x0 <= x1` and `y0 <= y1`. A negative `aspect_ratio` now produces exactly the same result as its
absolute value (e.g. `-1.0` behaves identically to `1.0`), which is the only sensible reading of
"aspect ratio" for sizing purposes (real width/height are always non-negative, so their ratio's
*sign* carries no separate shape information - only magnitude does). Positive-`aspect_ratio` inputs
are completely unaffected (`x.abs() == x` for `x >= 0`).

Because the project's own existing test suite hard-coded the previous (inverted) behavior as
"expected" for its one `aspect_ratio = -1.0` case in each of `contained_rect_with_aspect_ratio` and
`inscribed_rect_with_aspect_ratio`'s tests, the fix also updates those 2 lines' expected values
from `[0.0, 15.0, 10.0, 5.0]` (inverted) to `[0.0, 5.0, 10.0, 15.0]` (matches the existing
`aspect_ratio = 1.0` case for the same outer rect, as expected once only magnitude matters) - see
`patch.diff`.

Verified per `CONFIRMATION.md` section 7 (checked against more than the one triggering input):
`repro/` exercises 3 different inputs spanning both branches ("shrink x" on a square outer,
"shrink x" on a non-square outer with a different negative ratio, and "shrink y" with a third
negative ratio) plus one positive-ratio sanity case matching the existing doctest example - all
pass after the fix (see `output.txt`).

- Project test suite (`cargo test -p kurbo --lib`): 187 passed, 0 failed before the fix -> with
  only the production-code fix applied (test expectations left as-is): 185 passed, 2 failed
  (`rect::tests::contained_rect_with_aspect_ratio`, `rect::tests::inscribed_rect_with_aspect_ratio`,
  both failing because they asserted the old inverted result) -> with the 2 test expectations also
  corrected (full `patch.diff`): 187 passed, 0 failed (same count as baseline).
- Full workspace test suite (`cargo test --workspace`, includes doctests): 55 passed, 0 failed
  after the fix (not run as a separate "before" workspace-wide count; the `-p kurbo --lib` run
  above is the one that actually exercises this code and its tests).

## What's not done / open questions

- No confirmed real caller was found that ever passes a negative `aspect_ratio` to this function -
  this is a documented-domain gap (the doc comment doesn't forbid it, doesn't say what should
  happen for it, and the function doesn't panic/assert on it) rather than an observed failure from
  a real consumer. The value of this fix is making the function meet its own literal doc promise
  for a legal-per-the-type-signature input, not fixing an observed crash/misbehavior report.
  Lowering confidence accordingly, per the original candidate description.
  - Note the existing test suite already contains this case (`// negative aspect ratio`), which
  suggests it wasn't purely hypothetical to whoever wrote it - but there's no evidence (commit
  message, issue reference, doc comment) that the inverted result was a deliberate design choice
  as opposed to "the math produced this, and it wasn't checked for being a valid non-inverted
  rect."
- `Rect::width()`'s own doc explicitly says "nothing forbids negative width", so a maintainer could
  legitimately decide the *documented* behavior of `inscribed_rect_with_aspect_ratio` should be
  updated instead (e.g. explicitly stating the result may have negative width/height for a negative
  `aspect_ratio`, or that `aspect_ratio.abs()` is used) rather than changing the runtime behavior
  to always normalize - this candidate picked the latter (normalize) since it's what best satisfies
  the literal "largest possible Rect ... fully contained in self" wording already in the doc
  comment, but this is a legitimate design decision point for the fix, not a purely mechanical
  correctness fix.
- The sibling deprecated method `Rect::aspect_ratio()`/`contained_rect_with_aspect_ratio` calls
  straight through to `inscribed_rect_with_aspect_ratio` (`self.inscribed_rect_with_aspect_ratio(1.
  / inverse_aspect_ratio)`), so it inherits the same fix automatically for a negative
  `inverse_aspect_ratio` input - not a separately-checked code path.
- Candidate 031 (`RECT_ASPECT_TEST_HELPER_WRONG_HEIGHT_CHECK`) found the shared `case()` test
  helper's sanity-check `assert!` compares the wrong field; this candidate independently confirms
  (see "What was tried" row 3) that even a *corrected* magnitude-only sanity check would not have
  caught this specific bug, since the inverted result's `width()`/`height()` magnitudes are
  unchanged from the correct (non-inverted) result - only the sign/ordering of the coordinates
  differs. The two candidates are complementary, not overlapping.
