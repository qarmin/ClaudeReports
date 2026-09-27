# AFFINE_REFLECT_ZERO_DIRECTION_NAN

`Affine::reflect` (kurbo/src/affine.rs) computes a unit normal from the caller-supplied `direction`
via `Vec2::normalize()`, which is documented (kurbo/src/vec2.rs) to produce NaN when the input has
zero magnitude - but `reflect`'s own doc comment said nothing about this precondition, unlike the
analogous `Affine::inverse`/`TranslateScale::inverse`, which both explicitly document their NaN
footgun. Looked suspicious as a documentation-consistency gap noticed by direct inspection of
`affine.rs` (not from a static-analysis tool or fuzzer).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | Debug build, direct call `Affine::reflect((1.0,0.0),(0.0,0.0))` via public API | Reproduced immediately: `as_coeffs()` returns `[NaN; 6]`, matching the candidate's exact described repro |
| Repeat-run determinism check | Same debug build, 5 consecutive runs, no seed/RNG involved | 5/5 identical NaN output - deterministic pure floating-point computation |
| Source read of `Vec2::normalize`/precedent doc comments (`Affine::inverse`, `TranslateScale::inverse`) | N/A (reading, not executing) | Confirmed the project's own convention: NaN-producing functions here are documented, not guarded/panicking |

Given the finding is a deterministic, single-code-path floating-point computation (no `unsafe`,
no adversarial parsing, no concurrency), a debug build plus manual boundary-value construction was
sufficient on the first approach - no escalation to miri/ASAN/fuzzing was warranted (none of those
tools would add signal beyond what plain execution already showed).

## Status

**Confirmed**, 5/5 runs, debug build, deterministic (no RNG/seed).

**Fully reproducible**: **Yes** - the repro calls the real public `Affine::reflect` entry point
directly with a hand-constructed `(Point, Vec2)` argument pair. This is not "faking a second
participant" per `CONFIRMATION.md` section 11: the finding is about a single participant's own API
contract (a caller constructing arguments and calling a public function), not about how one
participant reacts to input actually produced by another - there is no peer/wire/upstream-caller
role being stood in for here.

## Class

documentation (a missing-precondition doc gap; the underlying NaN-producing computation itself is
intentional/unavoidable for a degenerate input, not a logic defect with a well-defined correct
alternative output)

## Root cause

`Affine::reflect(point, direction)` builds a perpendicular vector `Vec2 { x: direction.y, y:
-direction.x }` and calls `.normalize()` on it unconditionally. `Vec2::normalize()` divides by
`self.hypot()` (the vector's magnitude); when `direction` has zero length, the perpendicular vector
is also zero-length, so `normalize()` computes `0.0 / 0.0` component-wise, yielding NaN. Every
coefficient of the Householder reflection matrix constructed from `n.x`/`n.y` afterward is
therefore also NaN, and the returned `Affine`'s `as_coeffs()` is `[NaN; 6]`.

There is no well-defined "correct" reflection for a degenerate (zero-length) direction vector - the
line `point + direction * t` collapses to the single point `point` with no direction, so there is no
principled fallback output to invent (unlike some other confirmed candidates in this workspace, e.g.
032/040/041, where a well-defined fallback value existed and NaN was silently swallowing it). The
project's own existing convention for this exact situation - see `Affine::inverse`'s "Produces NaN
values when the determinant is zero" and `TranslateScale::inverse`'s "Produces NaN values when scale
is zero" doc comments - is to document the precondition rather than guard against it. `reflect`'s
own doc comment predates and does not follow this convention.

## Introduced by

80399cf "Implement reflection over a line -> affine transformation" (2023-06-08) - this commit
introduced `Affine::reflect` in its current form (found via `git log -S "pub fn reflect" --
kurbo/src/affine.rs`, a single matching commit). The doc comment has never mentioned the NaN
precondition since this function was first added.

## The fix

Added a one-line doc-comment note directly under `Affine::reflect`'s existing summary line,
matching the exact phrasing convention already used by `Affine::inverse`/`TranslateScale::inverse`:

```rust
/// Create an affine transform that represents reflection about the line `point + direction * t, t in (-infty, infty)`
///
/// Produces NaN values if `direction` has zero length.
///
/// # Examples
```

This is a documentation-only change - it does not alter what the function computes (the NaN output
for a zero-length `direction` is unavoidable and intentional per the root-cause analysis above, not
a defect to be silently patched over). Verified with the standalone `repro/`: the runtime behavior
(all-NaN `Affine` coefficients) is identical before and after the patch; what changes is that
`repro/`'s doc-comment check (reads the crate's own source and looks for "NaN" in the text directly
above `pub fn reflect`) now passes.

- Project test suite (`cargo test --all-features` inside `kurbo/`): 187 unit tests passed, 0 failed
  both before and after; 55 doctests passed, 0 failed both before and after. No regressions, as
  expected for a doc-comment-only change.

## What's not done / open questions

- No behavioral change was made (e.g. returning `Affine::IDENTITY` or panicking on a zero-length
  `direction`) - this was a deliberate choice to match the project's existing convention for
  identical situations elsewhere in the same file, not an oversight. A maintainer could instead
  choose to guard/panic if they consider silent NaN production for this specific public entry
  point worse than the precedent set by `inverse`; that would be a behavioral/API decision outside
  the scope of this session's bounded fix.
- Not checked: whether any other kurbo constructor with a similar "derive a unit vector from a
  caller-supplied direction" pattern has the same undocumented-NaN-precondition gap (a broader
  doc-comment sweep across the crate for this specific pattern was not performed this session).
