# INV_ARCLEN_NAN_SILENT_MIDPOINT

`ParamCurveArclen::inv_arclen`'s default trait method (`kurbo/src/param_curve.rs:98`) computes
`epsilon = accuracy / total_arclen` from the curve's own `self.arclen(accuracy)` before calling
`common::solve_itp`. When `total_arclen` is NaN (a curve with a non-finite/NaN control point, or
any other bug that makes `arclen()` return NaN), the guard `if arclen >= total_arclen { return 1.0
}` never triggers (NaN comparisons are always false), and the NaN silently gets absorbed by
`f64::max` deep inside `solve_itp_fallible`'s step-count computation, producing a plausible-looking
but meaningless finite return value (`0.5`) instead of propagating the NaN. Distinct from candidate
007 (SOLVE_ITP_FALLIBLE_SHIFT_OVERFLOW, a panic/hang in the same function family) - this is a
silent-wrong-result failure mode, and this is the only one of the three call sites of
`solve_itp`/`solve_itp_fallible` in this crate (the other two, `offset.rs:208` and
`fit.rs:627/692`, use hardcoded epsilon constants) whose `epsilon` is computed dynamically from a
value that can itself be NaN.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (debug build) | `CubicBez` with one control point at `f64::INFINITY`, `inv_arclen(1.0, 1e-6)` | Reproduced immediately: `arclen()` returns NaN, `inv_arclen` returns exactly `0.5` |
| Same, varied inputs (debug + release build) | `CubicBez` with a NaN control point directly; a normal finite curve with a NaN `arclen` query argument; a normal curve/query as a sanity check | All three NaN-producing variants reproduce identically (`0.5`); the sanity case behaves correctly in both builds |

No sanitizer/fuzzing was needed - this is a pure, deterministic floating-point logic bug fully
explained by reading `f64::max`'s documented NaN-handling semantics and manually constructing the
boundary-value input that exercises it; a debug build plus hand-picked NaN-producing curves was
sufficient on the first attempt, so no further escalation (miri/ASAN/fuzzing) was performed.

## Status

**Confirmed.** Reproduces 3/3 repeat runs, bit-for-bit identical output each time (this is a pure
deterministic floating-point computation with no randomness or threading, so full determinism is
expected and observed), in both debug and release builds. See `repro/` and `output.txt`.

**Fully reproducible**: **Yes**. `repro/src/main.rs` calls kurbo's real public API
(`CubicBez::inv_arclen`, via the `ParamCurveArclen` trait) directly with real, single-artifact
geometric input (a `CubicBez` with an infinite/NaN control point, or a NaN query argument) - there
is no second participant being faked here (see `CONFIRMATION.md` section 11): a `CubicBez` handed
straight to a public method is the genuine external input a real caller would construct, the same
way a malformed file handed to a parser's `from_reader` would be.

## Class

logic

## Root cause

`kurbo/src/param_curve.rs`, `ParamCurveArclen::inv_arclen`:

```rust
fn inv_arclen(&self, arclen: f64, accuracy: f64) -> f64 {
    if arclen <= 0.0 {
        return 0.0;
    }
    let total_arclen = self.arclen(accuracy);
    if arclen >= total_arclen {          // always false if either side is NaN
        return 1.0;
    }
    let mut t_last = 0.0;
    let mut arclen_last = 0.0;
    let epsilon = accuracy / total_arclen;   // NaN if total_arclen is NaN
    ...
    common::solve_itp(f, 0.0, 1.0, epsilon, 1, 0.2, -arclen, total_arclen - arclen)
}
```

If `self.arclen(accuracy)` returns NaN (which it does, for example, for a `CubicBez` with a
non-finite/NaN control point - `f64` arithmetic on an `Infinity`/`NaN` coordinate propagates NaN
through the Gauss-Legendre quadrature used by `CubicBez::arclen`), the `arclen >= total_arclen`
guard is `x >= NaN` which is always `false` per IEEE 754 comparison semantics, so execution falls
through instead of returning early. `epsilon = accuracy / NaN` is then NaN, and this NaN `epsilon`
is passed straight into `common::solve_itp` -> `solve_itp_fallible`
(`kurbo/src/common.rs:701-744`):

```rust
let n1_2 = (((b - a) / epsilon).log2().ceil() - 1.0).max(0.0) as usize;
```

`(b - a) / epsilon` is NaN, `.log2()` is NaN, `.ceil()` is NaN, `- 1.0` is NaN, and
`.max(0.0)` on a NaN receiver returns the *other* (non-NaN) operand per `f64::max`'s documented
semantics ("If one of the arguments is NaN, then the other argument is returned") - silently
turning the NaN into `0.0`. `n1_2` becomes `0`, `scaled_epsilon = epsilon * (1u64 << nmax) as
f64` is still NaN (NaN times anything is NaN), and the loop's own condition `while b - a > 2.0 *
epsilon` is `1.0 > NaN` which is again always `false`, so the loop body never executes and the
function returns the untouched initial bracket `(a, b) = (0.0, 1.0)`. Back in
`common::solve_itp`, this becomes `0.5 * (0.0 + 1.0) = 0.5` - an arbitrary, meaningless parameter
value silently returned to the caller instead of a NaN or an error, giving no indication that
anything went wrong.

The same mechanism reproduces for any input that makes `total_arclen` (or the `arclen` argument
itself) NaN, not just the one curve shape in the original finding - confirmed with three distinct
inputs (infinite control point, NaN control point, NaN query argument), all producing the same
bogus `0.5`.

## Introduced by

08962c1 "Fancier solver for inverse arclength" (2021-03-12, Raph Levien). This commit rewrote
`inv_arclen` from a plain bisection loop (which had no analogous "the guard doesn't fire for NaN"
gap, since the old code's per-iteration bounds updates handled NaN differently and always ran the
fixed iteration count `n`) to the current structure: it introduced the exact
`if arclen >= total_arclen { return 1.0; }` early-return guard, the
`epsilon = accuracy / total_arclen` computation, and the delegation to the newly-added
`common::solve_itp`/`solve_itp_fallible` (whose own `.max(0.0)` NaN-absorption has been present
unchanged since this same commit, per `git show 08962c1 -- src/common.rs`). Confirmed via
`git log -S "fn inv_arclen" --oneline -- src/param_curve.rs` (path history followed through the
later workspace-restructuring commit d67b9dd) and reading the full diff of 08962c1.

## The fix

Add an explicit NaN check right after computing `total_arclen`, before the `arclen >=
total_arclen` comparison that silently fails to catch it:

```rust
let total_arclen = self.arclen(accuracy);
if arclen.is_nan() || total_arclen.is_nan() {
    return f64::NAN;
}
if arclen >= total_arclen {
    return 1.0;
}
```

This directly targets the root cause (NaN reaching `solve_itp`'s internals via a comparison that
can't detect it) rather than papering over the one triggering curve shape - verified against three
different NaN-producing inputs in the same bug class (infinite control point, NaN control point,
NaN query argument), all of which now correctly return NaN instead of `0.5`, per
`CONFIRMATION.md` section 7. A fourth case (an ordinary finite curve/query) was also checked to
confirm the fix doesn't change behavior for the normal, non-NaN path.

- Project test suite (`cargo test --quiet` in the `kurbo` crate): 187 unit tests + 55 doctests
  passed before the patch; 187 unit tests + 55 doctests passed after the patch (242/242 total,
  both times, 0 failures either way).

## What's not done / open questions

- This candidate only addresses the specific NaN-propagation gap in `inv_arclen`'s own default
  trait method. It does not address *why* `CubicBez::arclen` (or any other curve's `arclen`) can
  return NaN in the first place for a non-finite control point - that's a separate, pre-existing
  behavior (arguably by design: kurbo generally does not validate that control points are finite),
  not something this candidate's fix changes.
- The other two `solve_itp`/`solve_itp_fallible` call sites in this crate (`offset.rs:208`,
  `fit.rs:627/692`) were not affected by this bug and were not modified, since both pass a
  hardcoded positive `epsilon` constant rather than one derived from a value that could itself be
  NaN - noted in the candidate description and confirmed by reading both call sites, but not
  independently re-audited for a different unrelated NaN-source in this session.
- No fuzzing/miri/sanitizer escalation was performed since manual boundary-value construction
  reproduced the bug deterministically on the first attempt (per `SKILL.md`'s tool-selection
  guidance for a "logic bug" classification) - a broader fuzz sweep over `ParamCurveArclen`
  implementations for other NaN-producing inputs was not attempted this session.
