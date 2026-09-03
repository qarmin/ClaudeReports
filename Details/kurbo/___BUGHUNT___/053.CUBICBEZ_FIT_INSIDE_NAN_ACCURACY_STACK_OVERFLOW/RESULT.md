# CUBICBEZ_FIT_INSIDE_NAN_ACCURACY_STACK_OVERFLOW

`CubicBez::fit_inside` (private helper, `kurbo/src/cubicbez.rs:325-334`) is reachable from the
public `CubicBez::approx_spline(accuracy)` API (via `approx_spline_n` -> `try_approx_quadratic` /
the `n > 1` loop body). It recurses via `self.subdivide()` guarded only by two comparisons against
`distance` and has no depth counter - looked suspicious because both guards (`hypot() <= distance`
and `mid.hypot() > distance`) are always `false` when `distance` is NaN under IEEE-754 comparison
semantics, which would make neither base case ever fire.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug build | `CubicBez::new((0,0),(0,100),(100,100),(100,0)).approx_spline(f64::NAN)`, `cargo run --example`, debug profile | Reproduced immediately: process aborts with "thread 'main' has overflowed its stack / fatal runtime error: stack overflow" |
| Same repro, release build | `cargo run --release --example` | Also reproduces (exit code 134/SIGABRT) - confirms this is genuine unbounded recursion, not a debug-assertions-only artifact |
| Repeat-run + a second, differently-shaped input in the same class | 3x repeat of the original repro (debug build) + a second curve with duplicate/collinear control points, both via NaN accuracy | All 3 repeat runs overflow identically; second input overflows identically before the fix, returns `None` after |

No escalation to miri/ASAN/fuzzing was needed or appropriate - `kurbo` has zero `unsafe` in
`kurbo/src` (see `___BUGHUNT___/README.md`), and the failure mode (infinite recursion driven by
NaN's comparison semantics) was reproducible on the first manually-constructed boundary value, so
the soft 3-approach cap was not spent hunting further.

## Status

**Confirmed**. Reproduces 3/3 runs in a debug build, and additionally in a release build (exit
code 134), both deterministically (no dependence on heap layout/ASLR/seed - NaN's comparison
semantics make the outcome the same on every run by construction).

**Fully reproducible**: **Yes**. The repro calls the real public `CubicBez::approx_spline` entry
point directly with `f64::NAN` - a single external f64 argument a real caller can pass with no
second participant to fake (see `CONFIRMATION.md` section 11: this is exactly the "single external
artifact, no second participant" case, not a peer/caller-interaction finding).

## Class

crash (stack overflow / process abort)

## Root cause

`fit_inside(&self, distance: f64) -> bool` (kurbo/src/cubicbez.rs:325-334) has exactly two base
cases, both implemented as direct floating-point comparisons against `distance`:

```rust
fn fit_inside(&self, distance: f64) -> bool {
    if self.p2.to_vec2().hypot() <= distance && self.p1.to_vec2().hypot() <= distance {
        return true;
    }
    let mid = ...;
    if mid.hypot() > distance {
        return false;
    }
    // Split in two.
    let (left, right) = self.subdivide();
    left.fit_inside(distance) && right.fit_inside(distance)
}
```

Under IEEE-754, any comparison (`<=`, `>`, `<`, `>=`) against NaN is `false`. When `distance` is
NaN, `hypot() <= distance` is always `false` (so the first `return true` never fires) and
`mid.hypot() > distance` is always `false` too (so the second `return false` never fires either).
With both base cases unreachable, the function falls through to `self.subdivide()` and recurses on
both halves every time, unconditionally, with no depth counter anywhere in the call chain - the
recursion only ends when the native call stack is exhausted.

`fit_inside` is called with the caller-supplied `accuracy` value, unvalidated, from two sites:
`try_approx_quadratic` (the `n == 1` case) and the loop body of `approx_spline_n` (the `n > 1`
case) - both reachable from the single public entry point `CubicBez::approx_spline(accuracy)`,
which is itself just `(1..=MAX_SPLINE_SPLIT).find_map(|n| self.approx_spline_n(n, accuracy))` with
no validation of `accuracy` before use. `MAX_SPLINE_SPLIT` (100) bounds the outer loop over `n`,
but does nothing for `fit_inside`'s own inner recursion, which is a separate, unbounded call tree
per `n`.

## Introduced by

43dacf3 "fit_inside function" (2021-09-07) - the commit that added this exact function body
verbatim (found via `git log -S "fn fit_inside"`); the two comparison-based base cases with no
depth counter have been unchanged since, confirmed by diffing the current file against this
commit's own addition.

## The fix

Added an explicit `distance.is_nan()` check as a new first statement in `fit_inside`, returning
`false` immediately (the same value the function already returns for any other input where the
mid-point check fails to certify a fit) before either comparison-based base case is reached:

```rust
fn fit_inside(&self, distance: f64) -> bool {
    // `distance` is NaN whenever every comparison against it is false, which would otherwise
    // make both base cases below unreachable and recurse via `subdivide()` without bound.
    if distance.is_nan() {
        return false;
    }
    if self.p2.to_vec2().hypot() <= distance && self.p1.to_vec2().hypot() <= distance {
        return true;
    }
    ...
```

This is sufficient (not just papering over the one triggering input) because NaN is the *only*
input value for which both of `fit_inside`'s existing base-case comparisons are simultaneously
false - every other real-valued `distance` (including negative and `+inf`) already terminates via
one of the two existing comparisons on the very first call, confirmed by re-deriving both
comparisons' truth tables for `distance ∈ {finite negative, 0, finite positive, +inf}` (an `+inf`
distance satisfies the first `<=` comparison immediately since `hypot()` is always finite for a
finite curve; a negative distance fails the first comparison but satisfies `mid.hypot() > distance`
immediately, since `hypot()` is always `>= 0.0 >` any negative number). Verified against a second,
differently-shaped curve (duplicate/collinear control points, still NaN accuracy) in addition to
the original triggering input - both now return `None` from `approx_spline` instead of overflowing
- and against a normal, non-NaN accuracy call on the same curve to confirm the fix doesn't change
behavior for valid input (see `output.txt`).

Effect on the public API: `CubicBez::approx_spline(f64::NAN)` now returns `None` (every `n` in
`approx_spline_n`'s `1..=MAX_SPLINE_SPLIT` loop fails to certify a fit, since `fit_inside` never
returns `true` for a NaN `distance`), instead of aborting the process.

- Project test suite (`cargo test --workspace --all-features`): 252 passed / 0 failed before the
  patch, 252 passed / 0 failed after (187 + 10 + 55 across the two crates' unit tests plus 1
  ignored doctest, unchanged) - see `output.txt` for the exact per-suite breakdown.

## What's not done / open questions

- Not checked: whether any other recursive helper in `kurbo` has the same "NaN makes all
  comparison-based base cases false" shape (e.g. other `subdivide`-based recursive helpers reachable
  from a public API with an unvalidated `f64` parameter). This candidate only fixes `fit_inside`
  itself; a sibling function with the same pattern elsewhere was not searched for.
- The fix returns `false` (fails the fit check) for NaN rather than, say, panicking or documenting
  the precondition - this matches the project's existing convention elsewhere in this workspace
  (candidates 011/018/024/040/041/043 etc. all treat NaN as "silently fails/rejects" rather than
  panicking), so no separate discussion with maintainers should be needed, but it is a choice
  (reject vs. document-only) worth flagging as such.
