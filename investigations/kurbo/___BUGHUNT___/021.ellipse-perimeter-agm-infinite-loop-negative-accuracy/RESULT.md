# ellipse-perimeter-agm-infinite-loop-negative-accuracy

`kurbo/src/ellipse.rs`'s `agm_elliptic_perimeter` (lines 397-456, reached from the public
`Shape::perimeter` impl for `Ellipse`, lines 264-287, whenever the ellipse isn't a degenerate line
and the cheap `kummer_elliptic_perimeter` series approximation isn't already accurate enough) runs
a `loop` whose only exit condition is `term <= accuracy * g` (line 439). This looked suspicious
because `accuracy` is a caller-supplied `f64` with no validation anywhere on the path from the
public API, and `term` is provably non-negative while `accuracy * g` is negative whenever
`accuracy < 0.0` (or NaN whenever `accuracy` is NaN), which reasoned out to a loop that could never
terminate for out-of-range accuracy values.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: call the public `Ellipse::perimeter` directly with a non-circular ellipse and `accuracy` set to representative out-of-range values (`-1.0`, `-1e-300`, `-inf`, `NaN`, and the boundary `0.0`/`-0.0`), wrapped in `timeout 5`, release build | `-1.0`, `-1e-300`, `-inf`, `NaN` all hang (killed by `timeout`, exit 124); `0.0` and `-0.0` do **not** hang (return the correct perimeter immediately) - see Root cause for why `0.0` is a special case | Confirmed: any `accuracy < 0.0` or NaN hangs; `accuracy == 0.0` does not |
| Same repro, debug build (`cargo build`, no `--release`) | `accuracy = -1.0` | Hangs identically (exit 124) - not a release-only optimization artifact |
| Repeat runs for determinism | `accuracy = -1.0`, release build, 3 consecutive runs | 3/3 hang identically - fully deterministic (pure floating-point computation, no randomness/timing dependence) |
| Root-cause instrumentation: temporary `eprintln!` inside the loop (reverted before writing the patch, not part of `patch.diff`) printing `term`/`accuracy*g`/`c`/`mul`/`a`/`g` every iteration | `accuracy = -1.0`, release build | Confirms the exact mechanism: `c` (and therefore `term`) hits bit-exact `0.0` at iteration 6 once `a`/`g` converge to the same `f64` value; `mul` then doubles every iteration forever, overflowing to `inf` around iteration ~1024-2000, after which `term` becomes `NaN` - see `output.txt` |

## Status

**Confirmed.** Reproduces deterministically 3/3 runs in both debug and release builds for
`accuracy = -1.0` (and independently confirmed for `-1e-300`, `-inf`, and `NaN` - any negative or
NaN `accuracy`, not just the one value first tried). This is not a memory-layout/seed-dependent
finding; it is a pure, deterministic floating-point control-flow bug that reproduces on every run
under the same input.

**Fully reproducible**: **Yes.** The repro calls the real public entry point
(`Ellipse::new(...).perimeter(accuracy)`, the actual `Shape::perimeter` trait impl a real caller
would invoke) directly - `Ellipse::perimeter` takes a single `f64` argument supplied entirely by
its caller in one process; there is no second participant (peer, network, upstream parser) whose
behavior would need to be faked to produce this input, so CONFIRMATION.md section 11's stricter
two-participant bar does not apply here (a single out-of-range float argument is itself the
complete external input, the same way a malformed file is a complete external input to a parser's
`open_file`).

## Class

crash (denial-of-service hang - not a panic/abort, but an unbounded busy loop with no way for the
caller to interrupt it short of killing the process/thread)

## Root cause

`agm_elliptic_perimeter` computes `let accuracy = accuracy / (2. * PI * x);` (still carries the
sign of the original argument, since `2. * PI * x` is always positive for a non-degenerate
ellipse) and then loops:

```rust
loop {
    let c2 = c.powi(2);
    let term = mul * c2;             // always >= 0 (mul and c2 both >= 0)
    ...
    if term <= accuracy * g {        // the ONLY break condition
        ...
        break;
    }
    mul *= 2.;
    c = (a - g) / 2.;
    ...
}
```

`term` is provably non-negative on every iteration (`mul` starts at `0.5` and only ever doubles;
`c2` is a squared value). The AGM sequence (`a`, `g`) converges quadratically, so within about 5
iterations `a` and `g` become bit-identical `f64` values, making `c = (a - g) / 2` exactly `0.0`
from that point on - and once `c == 0.0`, `term = mul * 0.0 == 0.0` forever, regardless of how
large `mul` grows.

- If `accuracy < 0.0`: `accuracy * g` is negative (since `g` stays positive, in `(0, 1]`), so
  `term (== 0.0) <= accuracy * g (< 0.0)` is always `false` - the loop can never break. `mul`
  keeps doubling every iteration (`mul *= 2.`) until it overflows `f64` to `+inf` (~1024-2000
  iterations later, since `mul` starts at `0.5` and doubles), at which point `term = inf * 0.0`
  becomes `NaN`; comparisons involving `NaN` are also always `false`, so the loop still never
  breaks - it spins forever.
- If `accuracy` is `NaN`: `accuracy * g` is `NaN` from the very first iteration, and any
  comparison against `NaN` is `false`, so the loop never breaks either, for the same reason.
- `accuracy == 0.0` (or `-0.0`) is *not* affected: once `term` reaches exactly `0.0`, the
  condition becomes `0.0 <= 0.0`, which is `true`, so the loop terminates normally on the first
  iteration where `c` underflows to `0.0`. This matches the empirical result above (`0.0`/`-0.0`
  do not hang) even though the candidate description's framing ("`accuracy <= 0.0`") suggested
  they might.

The public `Shape::perimeter` impl for `Ellipse` (lines 264-287) performs no validation of
`accuracy` at all before calling into this function (only `radii` is checked, for
finiteness/degenerate-line cases) - any `f64` a caller passes, including a negative value or `NaN`
from an upstream miscalculation, reaches this loop unfiltered.

## Introduced by

e8fa6c08 "Numerically approximate ellipse perimeter (#383)" (2024-10-06) - this commit introduced
`agm_elliptic_perimeter` (and the whole numerical AGM-based perimeter approximation) with the
unguarded `loop` as shown above; `git blame` on the affected lines and `git log --follow` on
`kurbo/src/ellipse.rs` both point to this commit as the one that added this exact code (no later
commit touched the loop's structure or its break condition).

## The fix

Bounded the loop's iteration count directly, since legitimate (positive, non-NaN) `accuracy`
values always converge (`term` hits exact `0.0`) within a handful of iterations thanks to the
AGM's quadratic convergence - the observed instrumentation above shows convergence by iteration 6
for a 2:1 radii ratio, and the mathematics (each AGM step roughly doubles the number of correct
bits, working from a 52-bit mantissa) bounds legitimate convergence to well under 64 iterations
regardless of eccentricity. Changed the infinite `loop { ... }` to `for _ in 0..64 { ... }`; the
final `2. * PI * x / a * sum` computation after the loop is unchanged and runs on whatever `a`/
`sum` values are current when the loop ends, whether by `break` (legitimate convergence) or by
exhausting the 64-iteration cap (pathological `accuracy`).

Verified:
- `repro/` re-run against the patched tree: `accuracy = -1.0`, `NaN`, and `-inf` all now return a
  finite perimeter value immediately instead of hanging (see `output.txt`, `just test`'s
  `reproduce-after` step).
- Checked more than just the one triggering input (CONFIRMATION.md section 7): `-1.0`, `NaN`, and
  `-inf` were all re-tested against the patched build, not just the exact `-1.0` value first
  found.
- Legitimate accuracy values are unaffected: `accuracy = 0.001` and `accuracy = 1e-12` (a valid,
  much stricter tolerance) both still return the correct perimeter (`9.688448219995387` and
  `9.688448220547667` respectively, against a reference value of `9.688448220547675` for this
  2:1-radii ellipse), matching their pre-patch values - the 64-iteration cap is never actually hit
  for real accuracy values, only for the pathological ones.
- Project test suite (`cargo test -p kurbo --release`), run in the isolated worktree used for this
  investigation, not the main project tree:
  - Before the patch: **185 passed; 2 failed** (`bezpath::tests::test_elements_to_segments_starts_on_closepath`,
    `bezpath::tests::test_must_not_start_on_quad`) - both pre-existing failures unrelated to this
    change (same failures reproduce on the pristine tree with `git stash` applied, i.e. with no
    changes at all; this matches already-tracked candidate 004,
    `___BUGHUNT___/004.SEGMENTS_LEADING_CLOSEPATH_PANIC/`, in this same workspace).
  - After the patch: **185 passed; 2 failed** - identical results, same two pre-existing failures,
    no new failures introduced.
  - `cargo test -p kurbo --release ellipse` specifically: **6 passed; 0 failed**, both before and
    after.

## What's not done / open questions

- No other numerically-iterative loop in `kurbo` was swept for the same "unbounded loop keyed on
  an unvalidated externally-supplied tolerance/accuracy parameter" pattern this session (e.g.
  other `accuracy`-taking public APIs like `CubicBez`/`BezPath` flattening or arc-length methods
  weren't specifically re-audited for the same class of issue) - this candidate covers
  `Ellipse::perimeter` only.
- The fix caps iterations rather than validating/rejecting non-positive or NaN `accuracy` at the
  public API boundary; a maintainer might instead prefer documenting that `accuracy` must be
  positive and finite, and/or clamping it early in `perimeter` itself (e.g.
  `accuracy.max(f64::MIN_POSITIVE)` or an explicit `debug_assert!`/early-return for
  non-positive/NaN accuracy) rather than a defensive iteration cap deep inside the private
  `agm_elliptic_perimeter` helper - both are valid approaches; the iteration cap was chosen here
  because it's minimal, self-contained to the function that actually has the unbounded loop, and
  doesn't change the documented behavior/signature of any public function.
