# RECT_ASPECT_TEST_HELPER_WRONG_HEIGHT_CHECK

`kurbo/src/rect.rs`'s `#[test]` helper `case()` shared by `contained_rect_with_aspect_ratio` and
`inscribed_rect_with_aspect_ratio` has a "sanity check" `assert!` meant to verify the computed
`expected` rect fits inside `outer` on both axes, but it compares `expected`'s width against
`outer`'s height a second time instead of comparing `expected`'s own height - flagged by a
suspicious repeated identifier (`expected.size().width` appearing twice) while reading the file
against `HEURISTICS.md`'s "copy-paste in near-duplicate assertions/branches" pattern.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (public API) | Copied the exact literal boolean expression from `rect.rs` (both instances) into a standalone program using kurbo's real `Rect`/`Size` public API, fed it `outer = Rect::new(0,0,10,10)` and a deliberately too-tall `expected = Rect::new(0,0,10,50)` (5x outer's height) | Original condition evaluates `true` (no panic) for the too-tall rect; a corrected condition (checking `expected.size().height.abs() <= outer.size().height.abs()`) evaluates `false` (would panic) for the same input - repeated 3/3 identical runs, `cargo run --release` |
| Mutation of the corrected assertion against the existing test corpus | Applied the one-line height-comparison fix to both `case()` instances in a debug build, ran `cargo test --lib rect::tests` | All 18 existing `rect::tests` still pass under the corrected assertion - confirms the current 24 test cases (12 per function) never actually exercised a rect whose height doesn't fit inside `outer`'s, i.e. no currently-hidden real bug in `contained_rect_with_aspect_ratio`/`inscribed_rect_with_aspect_ratio` was masked by this - the sanity check was simply dormant/toothless, not covering up an active regression |
| Manual trace of `inscribed_rect_with_aspect_ratio`'s two branches | Read the function body (`kurbo/src/rect.rs:581-608`) | The "shrink x" branch never touches `y0`/`y1` (height unchanged, trivially fits); the "shrink y" branch's `new_height = width * aspect_ratio.recip()` is smaller than the original height exactly when the branch's own guard condition (`self_aspect.abs() < aspect_ratio.abs()`) holds - so the production function's own logic doesn't currently produce an over-tall rect for any input the existing 24 cases explore, consistent with the mutation-test result above |

Only 2 of the soft cap of 3 approaches were tool-based digging; the third was code reading used to
corroborate the mutation-test result, not a separate escalation - stopped here since the first two
approaches already gave a clean, repeatable, non-flaky signal (this is a pure boolean-logic defect
in test code, not an adversarial-input class needing miri/ASAN/fuzzing).

## Status

**Confirmed** - deterministic, reproduces 3/3 runs (pure arithmetic on `f64`, no randomness, no
build-profile dependence: same result in `--release`).

**Fully reproducible**: **Yes**. This finding is not about one participant's reaction to another's
input (no peer/caller/wire relationship) - it's a single boolean-expression defect inside one
test helper function, evaluated directly through kurbo's own real public `Rect`/`Size` API. The
repro runs that exact real expression (copied verbatim from `rect.rs`, not paraphrased) end-to-end;
`CONFIRMATION.md` section 11's stand-in-second-participant concern doesn't apply here.

## Class

logic (test-only: a broken assertion inside the test suite itself, not a defect in the production
`contained_rect_with_aspect_ratio`/`inscribed_rect_with_aspect_ratio` implementations - see "What
was tried" row 2/3 above, no currently-hidden real bug was found).

## Root cause

Both `contained_rect_with_aspect_ratio` and `inscribed_rect_with_aspect_ratio`'s test `case()`
helper end with:

```rust
assert!(
    expected.size().width.abs() <= outer.size().width.abs()
        && expected.size().width.abs() <= outer.size().height.abs(),
    "Sanity check {expected} should be smaller than {outer}."
);
```

The second clause was clearly meant to check the height axis (mirroring the first clause's width
check) but instead re-reads `expected.size().width` a second time and compares it against
`outer.size().height` - a copy-paste slip where `width` was never changed to `height` on the left
side of the second `<=`. As a result the helper's "smaller than outer" sanity check never inspects
`expected`'s own height at all; any `expected` value with a correct width but an arbitrarily large
height passes silently, as demonstrated by `repro/` (a `50`-tall `expected` against a `10`-tall
`outer` passes the original condition, fails the corrected one).

## Introduced by

f90c15a "Add `aspect_ratio_width`, which uses the normal conventions for aspect ratios, deprecate
`aspect_ratio` (#486)" (2025-08-22) - `git log -S"expected.size().width.abs() <= outer.size().height.abs()"
-- kurbo/src/rect.rs` shows this is the only commit that ever touched this text; `git show` confirms
both `assert!` blocks (in `contained_rect_with_aspect_ratio` and `inscribed_rect_with_aspect_ratio`)
were added brand-new (`+` lines, no prior version existed) by this commit, so the flawed condition
was wrong from the moment the sanity check was first introduced, not a later regression of a
previously-correct check.

## The fix

Change the second clause's left-hand side from `expected.size().width` to `expected.size().height`
in both `case()` instances (`kurbo/src/rect.rs`, `contained_rect_with_aspect_ratio` test and
`inscribed_rect_with_aspect_ratio` test) - see `patch.diff` (2 lines changed, 1 per test function).
This makes the sanity check actually verify both axes, matching its own stated intent ("should be
smaller than {outer}").

Verified sufficient per `CONFIRMATION.md` section 7 (checked against more than just the one
triggering input): re-ran the full existing 24-case corpus (12 cases each for
`contained_rect_with_aspect_ratio`/`inscribed_rect_with_aspect_ratio`, covering squares, non-square
ratios, the identity ratio, negative ratio, `f64::INFINITY`/`0.0` ratios, and multiple
zero-dimension degenerate rects) against the corrected assertion - all still pass, so the fix does
not turn any currently-passing case into a false failure; and separately confirmed via `repro/`
that the corrected condition does reject a genuinely too-tall `expected` that the original
condition let through.

- Project test suite (`cargo test --lib`, whole `kurbo` crate): 187 passed, 0 failed before the fix
  -> 187 passed, 0 failed after the fix (identical pass count - the fix only tightens a test-only
  assertion that no existing case happened to violate).

## What's not done / open questions

- This defect is specific to these two `case()` helpers in `rect.rs`; a broader sweep for the same
  copy-paste-slip pattern (an assertion or branch that repeats one field name where a sibling field
  name was clearly intended) across the rest of the test suite was not performed this session -
  worth a separate, dedicated pass if that pattern is suspected to recur elsewhere.
- Because no currently-hidden bug in `contained_rect_with_aspect_ratio`/`inscribed_rect_with_aspect_ratio`
  was uncovered (see "What was tried" row 2/3), this candidate's value is entirely in tightening the
  test suite's own coverage going forward, not in fixing a live production defect - stated explicitly
  so this isn't mistaken for a functional/runtime bug in the deprecated/normal aspect-ratio APIs
  themselves.
