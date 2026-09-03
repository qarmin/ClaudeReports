# DASH_ZERO_PERIOD_NAN_SILENT_SOLID_LINE

`dash_iter` (`kurbo/src/stroke.rs:725`, backing the public `kurbo::dash`) computes the dash
pattern's period as `dashes.iter().sum()` and feeds it straight into `dash_offset.rem_euclid(period)`
and, for the initial dash-ix search loop, into repeated `dash_remaining += dashes[dash_ix]`
additions, with no check that `period` is actually a well-defined, strictly positive value. This
looked suspicious per the project's own repeated pattern (already the root cause of several other
confirmed candidates in this workspace, e.g. 041/043) of NaN silently satisfying `< 0.0`/`> 0.0`
comparisons as always-false and disabling a guard loop without any visible error.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (public `kurbo::dash` API), release build | `dash(line_0_to_100.into_iter(), 0.0, &[0.0, 0.0])` and 5 other non-empty, even-length, zero-or-negative-summing patterns | Every one collapsed to a single continuous, undashed `[MoveTo, LineTo]` - the dash pattern was silently ignored, matching the candidate description exactly |
| Same inputs, debug build | `cargo run` (debug) | Identical output to release - this is a `NaN`-comparison effect, not gated by `debug-assertions` |
| Repeat runs | 5 consecutive `cargo run --release` invocations of the exact `&[0.0, 0.0]` case | Identical output 5/5 |
| Widening the boundary-value search to the general "period not positive" class (not just exactly `0.0`) | `dash(..., &[-5.0, -5.0])` (period `-10.0`, no `NaN` at all - `rem_euclid` with a negative divisor is well-defined) | Found a second, more severe failure mode: this call **hangs forever** - `dash_iter`'s `while dash_remaining < 0.0 { dash_remaining += dashes[dash_ix]; ... }` search loop only ever adds more negative dash lengths, so it can never reach `>= 0.0` and never terminates. Confirmed with a 5s watchdog thread inside the standalone repro (see `repro/src/main.rs`), which reported `HANG DETECTED` every time |

No escalation to miri/ASAN/a fuzz harness was needed: both failure modes are deterministic,
reachable by direct construction of the public `dash()` API's own documented parameter types
(`dash_offset: f64`, `dashes: &[f64]`), and don't involve `unsafe`/FFI/pointer manipulation - this
is squarely the "logic bug, reason from an invariant and construct a violating input" case per
`HEURISTICS.md`.

## Status

**Confirmed**. The `period == 0.0` case (the candidate's own titular scenario) reproduces
deterministically 5/5 runs, both debug and release, with byte-for-byte identical output every
time (a pure `NaN`-comparison effect, no seed/layout dependency). The adjacent `period < 0.0`
hang, found while widening the boundary-value search around this same code, reproduces
deterministically 3/3 runs via the standalone `repro/`'s own watchdog-timeout mechanism (see
`output.txt` for all 3 before-fix and 3 after-fix runs) - every run hits the 5-second timeout
before the fix and returns immediately after it.

**Fully reproducible**: **Yes**. `repro/src/main.rs` calls the real, public `kurbo::dash(inner,
dash_offset, dashes)` entry point directly with a real `BezPath`'s `PathEl` iterator and a plain
`&[f64]` dash-pattern argument, exactly as any real caller would construct one. There is no second
participant to fake here: `dashes: &[f64]` is a plain function argument the caller supplies
directly (the same single-artifact-input carve-out `CONFIRMATION.md` section 11 and this
project's candidate 002 already used for this same function), not something produced by a
peer/upstream caller over some channel.

## Class

logic

## Root cause

`dash_iter` (`kurbo/src/stroke.rs:725-751` before the fix):

```rust
let period = dashes.iter().sum();
let period = if dashes.len() % 2 == 1 { 2.0 * period } else { period };
let dash_offset = dash_offset.rem_euclid(period);

let mut dash_ix = 0;
let mut dash_remaining = dashes[dash_ix] - dash_offset;
let mut is_active = true;
while dash_remaining < 0.0 {
    dash_ix = (dash_ix + 1) % dashes.len();
    dash_remaining += dashes[dash_ix];
    is_active = !is_active;
}
```

`period` is never checked to be strictly positive before being used. This has two distinct failure
modes depending on which non-positive value it takes:

- **`period == 0.0`** (any non-empty, even-length `dashes` whose entries sum to exactly zero - all
  zero entries, or entries that cancel like `[-3.0, 3.0]`): `x.rem_euclid(0.0)` is IEEE-754 `NaN`
  for any `x` (including `x == 0.0`, since `0.0 % 0.0` is itself `NaN`). `dash_remaining` becomes
  `NaN` and stays `NaN` through the rest of the computation (`dash_remaining -=
  self.seg_remaining` in `DashIterator::step()` keeps it `NaN`). Both the `while dash_remaining <
  0.0` loop here and the `self.dash_remaining < self.seg_remaining` branch check inside `step()`
  compare against `NaN`, which is always `false` in IEEE-754 - so `is_active` (initialized `true`)
  is never toggled anywhere, and `step()` always takes its "emit the whole segment" branch. The
  entire path is emitted as one continuous segment with no gaps: the dash pattern is silently
  ignored and the caller gets a solid line back, with no panic, no error, and (as far as could be
  observed in this session) no visibly-`NaN` coordinate anywhere in the output path, since the
  `NaN` never reaches any point-computing code path (the branch that would use it,
  `seg.inv_arclen(self.dash_remaining, ...)`, is never taken).
- **`period < 0.0`** (e.g. `[-5.0, -5.0]`, all-negative entries - no `NaN` at all here, since
  `rem_euclid` with a negative divisor is well-defined for finite inputs): the `while
  dash_remaining < 0.0` search loop's own termination assumption is that adding `dashes[dash_ix]`
  eventually pushes `dash_remaining` to `>= 0.0`. When every entry in `dashes` is negative, each
  iteration only makes `dash_remaining` more negative, so the loop **never terminates** - a genuine
  unconditional infinite loop / hang (confirmed by the 5-second watchdog thread in `repro/`).

Both stem from the same missing precondition check at the same code location, just triggered by
different subsets of the same invalid-input space (`period <= 0.0`).

Whether the `period == 0.0` case's *visible output* (a solid line) is itself "wrong" is genuinely
debatable: SVG2's own `stroke-dasharray` spec treats a dash array whose sum is zero as equivalent
to `none` (i.e. render as a solid line) - which is exactly what this code already does, just by
accident, via `NaN` comparisons happening to all evaluate `false` in a way that keeps `is_active`
permanently `true`. This session did **not** find any input where the `period == 0.0` case's
*visible* output actually diverges from that same solid-line fallback (see `output.txt`'s
"NOTE ON CLASS 1's UNCHANGED VISIBLE OUTPUT"). What is unambiguously a real defect, independent of
that debate, is (a) the `period < 0.0` hang, a genuine, severe, unconditional denial-of-service
reachable through the same public API and the same missing check, and (b) the `period == 0.0`
case relying entirely on an undocumented, untested, accidental interaction between two unrelated
`<` comparisons both happening to treat `NaN` the same way - nothing in the code enforces or tests
this, so a future, well-intentioned change to either comparison (or to how `dash_remaining` is
computed) could silently break it with no test to catch the regression.

## Introduced by

958c8ec "Properly handle negative dash offset by normalization (#454)" (2025-06-26). Found via
`git log -S "dashes.iter().sum()" -- kurbo/src/stroke.rs`, which returns exactly this one commit -
it's the commit that introduced the `period`/`rem_euclid` normalization this candidate is about.
Confirmed by reading its diff: before this commit, `dash_offset` was used directly with no
`period`/`rem_euclid` step at all, so `dashes = [0.0, 0.0]` with the crate's common `dash_offset =
0.0` default would have taken `dash_remaining = dashes[0] - 0.0 = 0.0` (not `< 0.0`, loop skipped,
no `NaN` anywhere) - the exact `x.rem_euclid(0.0)` mechanism this candidate describes did not
exist before this commit.

The adjacent `period < 0.0` infinite loop is not attributed to this same commit - the `while
dash_remaining < 0.0 { dash_remaining += dashes[dash_ix]; ... }` search loop itself (whose
termination assumption breaks for all-negative `dashes`) predates 958c8ec and was not
specifically bisected this session (see "What's not done" below).

## The fix

Guard the `period > 0.0` precondition explicitly at the one place `period` is computed, instead of
letting a non-positive value flow into `rem_euclid` and the search loop:

```rust
let (dash_ix, dash_remaining, is_active) = if period > 0.0 {
    let dash_offset = dash_offset.rem_euclid(period);
    let mut dash_ix = 0;
    let mut dash_remaining = dashes[dash_ix] - dash_offset;
    let mut is_active = true;
    while dash_remaining < 0.0 {
        dash_ix = (dash_ix + 1) % dashes.len();
        dash_remaining += dashes[dash_ix];
        is_active = !is_active;
    }
    (dash_ix, dash_remaining, is_active)
} else {
    (0, f64::INFINITY, true)
};
```

`period > 0.0` is `false` for `period == 0.0`, any negative `period`, and `NaN` (a `NaN` period
could arise from e.g. mixed infinities in `dashes`, out of this candidate's own scope but caught
for free by the same guard). The `else` branch never evaluates `rem_euclid` or the search loop at
all, and never indexes `dashes[0]` either - as a side effect, this also means the public `dash()`
entry point no longer panics on an **empty** `dashes` slice (candidate 002's own bug), since an
empty array's `sum()` is `0.0`, which now takes the same `else` branch instead of unconditionally
evaluating `dashes[0]`. `f64::INFINITY` as `dash_remaining` makes every later `dash_remaining <
seg_remaining` comparison in `DashIterator::step()` deterministically `false` (matching, by
explicit construction rather than by `NaN` accident, the same "always emit the whole segment"
behavior the pristine code produced only by chance for `period == 0.0`).

Verified against more than the one triggering input, per `CONFIRMATION.md` section 7: re-ran the
patched binary against all 6 `period == 0.0`-class inputs from the "What was tried" table (all-zero
even-length, all-zero 4-element, negative-canceling in both orders, nonzero offset, odd-length
doubled-to-zero) and both `period < 0.0` inputs - every one produced the correct/expected result
(solid-line passthrough, no hang) - see `output.txt`.

- Before -> after (`repro/`, run via `just test` from a fresh clone at the base commit, and
  directly against the local worktree): `period == 0.0` cases unchanged in visible output (solid
  line both before and after - the fix changes the *mechanism*, not this specific case's *result*,
  see root cause above); `period < 0.0` cases: hang (5s watchdog timeout, 3/3 runs) -> completes
  instantly (3/3 runs). See `output.txt` for full captured output of both.
- Project test suite (`cargo test --release` in `kurbo/`): **185 passed; 2 failed both before and
  after** the patch - the 2 failures (`bezpath::tests::test_elements_to_segments_starts_on_closepath`,
  `bezpath::tests::test_must_not_start_on_quad`) are pre-existing on this exact commit and
  unrelated to `stroke.rs` (already independently observed by several other candidates in this
  workspace, e.g. 011/021/022/023/032). The `stroke::` module's own 14 tests: **14 passed; 0
  failed**, identical before and after.

## What's not done / open questions

- The `period < 0.0` infinite-loop hang was found incidentally while widening this candidate's own
  boundary-value search, and this session's fix happens to close it too (same guard, same code
  location) - but it was not independently root-caused with its own `git bisect`/full
  `CONFIRMATION.md` treatment as a standalone candidate (no dedicated `NNN.NAME/` folder of its
  own). A future session may want to give it that treatment explicitly (its own repro focused
  purely on the hang, its own "introduced by" lookup for the search-loop's termination assumption
  specifically, which predates the commit identified above).
- Whether `period == 0.0`'s solid-line fallback is the *documented, intended* contract for
  `kurbo::dash` (as opposed to an accident that happens to match it) was not settled - the crate's
  own doc comment on `pub fn dash` doesn't mention this case at all. This session's fix keeps the
  existing observable behavior for that case (matching the SVG2 "sum of zero renders as none"
  convention) rather than changing it, on the theory that a silent, deterministic passthrough is
  strictly better than either a silent `NaN`-mediated passthrough or introducing new panicking
  behavior for an input class kurbo previously tolerated.
- Individually negative dash-length entries whose *sum* is still positive (e.g. `[10.0, -5.0]`,
  period `5.0`) were not investigated this session - `period > 0.0` there, so the new guard doesn't
  engage, and whatever behavior the pristine `while` loop already had for that input is unchanged.
- No fuzzing was run over the full `(dashes: Vec<f64>, dash_offset: f64)` input space; this session
  relied on manual boundary-value construction targeted at the "period not positive" invariant
  specifically. A short fuzz harness over arbitrary small `f64` vectors (including infinities/NaN
  entries) might turn up further related cases in the same function.
