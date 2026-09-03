# MINDIST_S_CORNER_NAN_SWALLOWED_BY_MIN_REDUCE

`kurbo/src/mindist.rs:50` (`min_dist_param`) computes the branch-and-bound pruning bound `alpha`
via `svalues.iter().map(|(a, _, _)| *a).reduce(f64::min).unwrap()` over 4 corner evaluations of
`S(u,v,bez1,bez2)`. Since `f64::min(NaN, x) == x`, the candidate hypothesized that a single NaN
corner (plausible given already-confirmed candidates 006/019's `mindist.rs` integer-overflow
findings for degenerate/high-degree control points) could be silently dropped from the reduce,
corrupting `alpha` into a wrong-but-plausible finite value instead of visibly propagating to a
NaN result - a stealthier failure mode than 006/019's own "whole result becomes NaN" finding.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: temporary in-crate `#[test]` in a scratch worktree calling the real, private `S()` directly (accessible from `mindist.rs`'s own `mod tests`), fixed shape, `n` scanning 4..40 across the debug-assertions `choose()`-overflow threshold candidate 019 already found (~16) | `cargo test --lib`, dev profile (debug-assertions=true) and `--release` | Debug: panics at `mindist.rs:252:9` ("attempt to multiply with overflow") for `n>=16`, before any `S()` value is ever observed - matches candidate 019 exactly, no new signal. Release: for `n<16`, all 4 corners are finite; for `n>=16`, **all 4 corners are `NaN` together, every time** - never a mix of NaN and finite corners. |
| Same real, private `S()`, broadened to a randomized sweep: 480 cases (`bez1` degree 1..40, `bez2` degree in {1,3,5,40}, 3 different `(u,v)` subdivision-rectangle shapes including post-subdivision interior corners, not just the initial `[0,1]` domain corners), pseudo-random control points via an inline xorshift generator | `cargo test --release --lib`, ~17s | `any_mixed=false` across all 480 cases - the 4 `svalues` corners were always uniformly all-NaN or all-finite, never a mix, at any degree or subdivision-rectangle shape tried. |
| Real, end-to-end `min_dist_param` (`pub(crate)`, the one real reachable entry point) via the standalone `repro/` (`#[path]`-vendored, unmodified `mindist.rs` from a pristine clone at `base_commit`), `n` scanning 4..64 straddling the overflow threshold | `just test` (both debug and `--release`), 3/3 repeat runs, identical output every time | Debug: panics for `n>=16` (candidate 019's known overflow), never reaches a returned value. Release: `alpha` is a finite plausible number for `n<16`, and exactly `NaN` (never a finite-but-wrong value) for every `n>=16` tested (16, 17, 18, 20, 24, 32, 40, 64) - see `output.txt`. |

## Status

**Not reproducible.**

Covered: 3 escalating approaches (direct-function boundary-value probing, a 480-case randomized
sweep of the same private function across degree/subdivision-rectangle shapes, and a real
end-to-end run of the actual `pub(crate)` entry point via the standalone `repro/`), all pointing
at the same structural conclusion below. Not an exhaustive proof over all possible inputs (no
formal proof was constructed, and the randomized sweep is a sample, not exhaustive), but the
mechanism ruling this out (see "Root cause") is a general property of the code's data flow, not
specific to the sampled inputs, so a wider sweep is not expected to change the conclusion.

**Fully reproducible**: N/A (Not reproducible - no bug to reproduce). The `repro/` in this folder
demonstrates the negative result via the real, unmodified, reachable `min_dist_param` entry point
directly (not a hand-built stand-in) - see `CONFIRMATION.md` section 11's "single-process,
single-participant repro remains completely fine" carve-out; there is no second participant to
fake here (this is a pure, single-process numerical computation).

## Class

N/A (Not reproducible)

## Root cause (why the suspect turns out to be safe)

`S(u,v,bez1,bez2)` sums `D_rk(r,k,bez1,bez2) * basis_function(2n,r,u) * basis_function(2m,k,v)`
over all `(r,k)`. Critically, **`D_rk` does not take `u`/`v` as arguments at all** - it depends
only on `(r,k,bez1,bez2)`, so its value (and whether it's NaN) is identical across all 4
`svalues` corners for a given `(bez1,bez2)` pair.

IEEE-754 arithmetic propagates NaN unconditionally through multiplication and addition -
`NaN * 0.0 == NaN`, not `0.0` - so if `D_rk(r,k)` is NaN for even one `(r,k)` pair (the mechanism
candidates 006/019 already confirmed, via `choose()`'s `u32` overflow), the term
`D_rk(r,k) * basis_function(...) * basis_function(...)` is NaN regardless of what the two
`basis_function` factors evaluate to at that corner - even if one of them happens to be an exact
`0.0` (which does occur at the domain-boundary corners `u=0`/`u=1`, where all but one `r`'s basis
value is exactly zero). Since `S`'s summation is a running `+=`, once any single term is NaN the
whole sum is NaN, for every corner, unconditionally.

This means the 4 values fed into `reduce(f64::min)` can never actually be "3 finite + 1 NaN" -
they are always either all-finite (no `D_rk` term is NaN) or all-NaN (some `D_rk` term is NaN,
which then poisons every corner identically). `reduce(f64::min)` over an all-NaN input correctly
produces `NaN` back out (`f64::min` returns `NaN` when neither operand is a preferable non-NaN
value), matching the already-documented candidates 006/019 behavior ("the whole result becomes
NaN"), not the stealthier "one corner silently dropped, wrong finite `alpha` survives" failure
this candidate hypothesized. The `reduce(f64::min)` call is not itself buggy in a way that's ever
actually exercised, because its precondition for the hypothesized bug (a genuine per-corner-only
NaN) never arises given `D_rk`'s `u,v`-independence.

## Introduced by

N/A (Not reproducible; no bug to attribute). For reference, `svalues`/the `reduce(f64::min)` call
and `S`'s summation structure were both part of 81fbd069 "Compute the minimum distance between two
pathsegs." (2021-08-25, the same commit candidates 006/019 point to for the underlying
integer-overflow mechanism this candidate's hypothesis depended on).

## The fix

Not applicable - no reproducible defect found this session, so no patch was written.

- Project test suite: not run against a patch (none was written; no source file in the main
  project was modified this session - all probing was done in scratch worktrees/the standalone
  `repro/`). N/A.

## What's not done / open questions

- This candidate's negative result depends on `D_rk` never taking `u`/`v` as input, which is a
  structural fact of the current code, not something that could change silently - but if a future
  refactor ever made any term inside `S`'s summation `u`/`v`-dependent in its NaN-producing
  behavior (not just its finite value), this conclusion would need re-checking.
  Already-confirmed candidates 006 (`mindist.rs` `2*n` `u8` overflow) and 019 (`choose()` `u32`
  overflow) remain the actual, real way `S()`/`min_dist_param` produce NaN at all - this candidate
  only rules out one specific additional failure mode (`reduce(f64::min)` silently swallowing a
  partial-NaN corner) layered on top of those, it does not re-litigate whether 006/019 themselves
  are real (they are, and remain unfixed in the current source, per this skill's "never leave the
  main tree dirty" rule).
- The randomized sweep (480 cases) is a sample over degree 1-40 (`bez1`) x {1,3,5,40} (`bez2`) x 3
  subdivision-rectangle shapes, not an exhaustive search over the full `u8` degree range (up to
  254, per candidate 006's own tightened boundary) or over arbitrary `(u,v)` rectangle shapes -
  a longer/dedicated fuzzing campaign (via `fuzz-bug-repro`, run separately) could widen this, but
  is not expected to find a counterexample given the structural argument above.
- Not checked whether `f64::min`'s specific behavior when *both* reduce operands are NaN (as
  opposed to a genuine, hypothesized single-NaN case) could itself have a different subtlety in
  some other calling context elsewhere in the crate - out of scope for this candidate, which was
  specifically about `mindist.rs`'s own 4-corner `svalues` reduce.
