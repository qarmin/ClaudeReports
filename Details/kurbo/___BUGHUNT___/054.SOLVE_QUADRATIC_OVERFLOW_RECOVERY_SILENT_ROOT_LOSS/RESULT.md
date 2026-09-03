# SOLVE_QUADRATIC_OVERFLOW_RECOVERY_SILENT_ROOT_LOSS

`kurbo::common::solve_quadratic` (`kurbo/src/common.rs:246-289`) has an overflow-recovery branch
(`!arg.is_finite()`, lines 262-265) that assumes the overflow of `sc1*sc1 - 4.*sc0` always comes
from `sc1*sc1` itself overflowing, and always approximates one root as `-sc1`. This looked
suspicious because it never checks whether the *other* term (`4.*sc0`) could be the one that
actually overflowed instead - which happens whenever `sc1` (== `c1/c2`) is zero or small while
`sc0` (== `c0/c2`) is large enough in magnitude (~`f64::MAX/4`) for `4.*sc0` alone to overflow.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + direct call to the public `solve_quadratic` API | debug build, `solve_quadratic(c0=-1e308, c1=0.0, c2=1.0)` (a few one-off runs) | Reproduced immediately: returned `[-0.0]` instead of the two real roots `+-1e154`; `-0.0` is not even an approximate root (residual `-1e308`, i.e. 100% relative error) |
| Repeat-run determinism check (same debug build, same input, 3x) | debug build, same input as above | Bit-for-bit identical wrong output all 3 runs |
| Standalone `repro/` against a pristine clone at `base_commit`, release build | `cargo run --release`, same input | Reproduced identically (exit code 1); after applying `patch.diff`, reproduced correctly (exit code 0, both roots recovered with zero residual) |

Only one approach was needed (manual boundary-value construction fits this bug class exactly per
`HEURISTICS.md`'s "arithmetic on untrusted magnitudes" guidance) - it produced an immediate,
deterministic, unambiguous signal, so no escalation to a second/third tool was necessary.

## Status

**Confirmed**. Reproduces deterministically 3/3 runs (debug build) plus again via the standalone
`repro/` in both debug (implicit, during initial digging) and release builds against a pristine
clone of the project at `base_commit` - same wrong output (`[-0.0]`) every time, no flakiness, no
dependency on memory layout/seed/thread scheduling.

**Fully reproducible**: Yes. `repro/` calls `kurbo::common::solve_quadratic` directly, which is a
`pub fn` - the project's real, documented public API, and the exact function every internal caller
(`QuadBez::extrema`, `CubicBez::inflections`, `offset.rs`, `stroke.rs`, `fit.rs`, `bezpath.rs`) goes
through. This is a pure, single-participant numerical routine (no peer/wire/upstream caller
involved) - CONFIRMATION.md section 11's "real second participant" bar concerns a different kind of
finding and does not apply here; calling the public function directly with the triggering
coefficients *is* the real end-to-end trigger path.

## Class

logic

## Root cause

`solve_quadratic` normalizes coefficients as `sc0 = c0/c2`, `sc1 = c1/c2`, then computes the
discriminant `arg = sc1*sc1 - 4.*sc0`. When `arg` overflows to a non-finite value, the code's
comment assumes this is "likely" because `sc1*sc1` itself overflowed (true only when `|sc1|` is
astronomically large, roughly `> 1.34e154`), and unconditionally sets `root1 = -sc1` - the root of
the *linear-term-dominated* approximation `sc1*x + x^2 = 0` (i.e. treating `sc0` as negligible).

But `arg` can just as easily overflow because `4.*sc0` alone overflows (`|sc0|` roughly
`> 4.5e307`), while `sc1` is zero or comparatively tiny. In that regime the *quadratic term*
dominates instead: the correct approximation is `x^2 + sc0 = 0`, giving roots `+-sqrt(-sc0)` (or no
real roots if `sc0 > 0`) - the opposite of what the code assumes.

With `sc1 = 0.0` and `sc0 = -1e308`: `root1 = -sc1 = -0.0`. Then `root2 = sc0/root1 = -1e308 / -0.0
= +inf`, which fails `root2.is_finite()` and is discarded, leaving `result = [-0.0]` - a single
"root" that isn't even close to satisfying the original equation (plugging `x=-0.0` back in gives
`c0 + c2*x*x = -1e308`, not `~0`), instead of the two genuinely real roots near `+-1e154`.

This is a distinct code path from candidate 037 (that candidate's `arg < 0.0` branch is the
opposite discriminant-sign case, reached only when `arg` computes to a finite negative value) and
from candidate 018 (which established `solve_quadratic` cannot itself emit `NaN`, but did not
examine whether the overflow-recovery branch could still silently drop or corrupt a real root -
which it does, as shown here, without ever producing a `NaN`).

## Introduced by

8d0c810 "Make quadratic solver numerically robust" (2019-09-03) - this commit introduced the
overflow-recovery branch verbatim, including its comment "Likely, calculation of sc1 * sc1
overflowed" and the unconditional `-sc1` approximation, as part of an initial numerical-robustness
pass. The branch has not been touched since (confirmed via `git log -S`/`git blame` on
`kurbo/src/common.rs`; the only later touches to the file are unrelated moves/refactors: d67b9dd
workspace restructuring, b85607f clippy lint fix, 6c24b98 `rem_euclid` relocation, 021e291
`solve_itp` dedup).

## The fix

Distinguish which term actually overflowed before choosing the approximation: if `sc1*sc1` alone
is still finite (so `4.*sc0` must be the term that overflowed), the quadratic term dominates -
return `+-sqrt(-sc0)` directly (or no roots if `sc0 > 0.0`, since `x^2 = -sc0` would be negative).
Only fall through to the old `-sc1` approximation when `sc1*sc1` itself is non-finite, i.e. `|sc1|`
is the dominant/overflowing quantity.

```
--- a/kurbo/src/common.rs
+++ b/kurbo/src/common.rs
@@ -260,8 +260,24 @@ pub fn solve_quadratic(c0: f64, c1: f64, c2: f64) -> ArrayVec<f64, 2> {
     }
     let arg = sc1 * sc1 - 4. * sc0;
     let root1 = if !arg.is_finite() {
-        // Likely, calculation of sc1 * sc1 overflowed. Find one root
-        // using sc1 x + x² = 0, other root as sc0 / root1.
+        // The subtraction overflowed. Which term dominates determines which
+        // approximation is valid: if sc1 * sc1 alone is still finite, then it
+        // was the 4 * sc0 term that overflowed, meaning |sc0| >> sc1², so the
+        // linear term is negligible and the roots come from x² + sc0 = 0
+        // instead (using sc1 here, as the old code unconditionally did,
+        // would silently produce a bogus root - e.g. 0 - when sc1 is 0 or
+        // small while sc0 is huge).
+        if (sc1 * sc1).is_finite() {
+            if sc0 > 0.0 {
+                return result;
+            }
+            let r = (-sc0).sqrt();
+            result.push(-r);
+            result.push(r);
+            return result;
+        }
+        // Otherwise sc1 * sc1 itself overflowed, so |sc1| dominates: find one
+        // root using sc1 x + x² = 0, other root as sc0 / root1.
         -sc1
     } else {
         if arg < 0.0 {
```

Verified against the triggering input plus several variants (before -> after):
`solve_quadratic(-1e308, 0.0, 1.0)`: `[-0.0]` (residual `-1e308`) -> `[-1e154, 1e154]` (residual
`0.0` both); `solve_quadratic(-1e300, 1e10, 1.0)` (small nonzero `sc1`, `sc0`-dominated overflow):
also recovers both roots correctly after the fix; `solve_quadratic(1e308, 0.0, 1.0)` (`sc0 > 0`,
`4.*sc0` overflows, genuinely no real roots since `x^2 = -sc0 < 0`): pre-fix returns `[-0.0]` (also
wrong - a spurious root), post-fix correctly returns `[]`. The pre-existing `sc1`-dominated
overflow case (large `|sc1|`, e.g. `solve_quadratic(1.0, 1e200, 1.0)`) is untouched by the fix and
produces identical output before/after (`[-1e200, -1e-200]`).

- Project test suite (`cargo test` in `kurbo/`, the package containing the changed function):
  **187 passed; 0 failed** before the patch, **187 passed; 0 failed** after the patch (both debug
  builds). Full workspace `cargo test` (including `polycool` and all doctests) also passed with no
  failures both before and after.

## What's not done / open questions

- `solve_cubic` (same file) delegates to `solve_quadratic` when its own cubic-coefficient-overflow
  guard fires, so it inherits this fix automatically for that path - not separately re-verified
  with its own dedicated extreme-magnitude cubic input this session.
- The fix does not address the case where both `sc1*sc1` and `4.*sc0` individually overflow (a
  genuinely ill-conditioned near-double-root case at extreme magnitude) - that combination was not
  constructed or tested this session; the fix only disambiguates which single term is responsible
  when exactly one of the two has overflowed.
