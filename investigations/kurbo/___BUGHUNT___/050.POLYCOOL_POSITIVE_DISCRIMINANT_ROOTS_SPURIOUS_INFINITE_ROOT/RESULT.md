# POLYCOOL_POSITIVE_DISCRIMINANT_ROOTS_SPURIOUS_INFINITE_ROOT

`polycool/src/quadratic.rs`'s `Quadratic::positive_discriminant_roots` (lines 83-98) computes
`r0 = q / a`, `r1 = c / q` and returns `Some((r0.min(r1), r0.max(r1)))` unconditionally whenever
`disc > 0.0`, with no finiteness check on `r0`/`r1` - unlike its sibling `roots()` (lines 26-79),
which explicitly filters non-finite roots via a `push` closure checking `r.is_finite()` to handle
the degenerate `a == 0` (linear-equation) case. This looked suspicious because the two functions
compute the exact same `q`/`r0`/`r1` from the exact same coefficients but only one of them guards
against infinity.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (a == 0, degenerate/linear case) + debug-assertions build, temporary `#[test]` in `polycool/src/quadratic.rs` | `Quadratic::new([1.0, 2.0, 0.0])` (2x + 1 = 0), `cargo test -p polycool --lib`, debug profile | Reproduced immediately: `roots()` correctly returns `[-0.5]`, `positive_discriminant_roots()` returns `Some((-inf, -0.5))`. Ran 3 times, identical result every time (deterministic float arithmetic, no randomness/threading). |
| Standalone `repro/` (real public API, `--release`), via `just test` in this candidate's own folder, against a fresh clone of the repo at the base commit | `cd repro && cargo run --release` (repro depends on the real `polycool` crate via a path dependency into `./clone/polycool`) | Same result: `Some((-inf, -0.5))` before the patch, `None` after applying `patch.diff`. |
| Varied the triggering input (second `a == 0` polynomial with different `b`/`c`) plus a normal, non-degenerate quadratic as a regression check | `Quadratic::new([-6.0, 3.0, 0.0])` (3x - 6 = 0) and `Quadratic::new([2.0, -3.0, 1.0])` (x^2 - 3x + 2 = 0), same test harness, patched tree | Degenerate case: `None` (fix applies). Normal case: `Some((1.0, 2.0))`, unchanged - confirms the fix doesn't affect the well-conditioned path. |

Only 2 of the soft cap of 3 approaches were needed - the bug reproduced deterministically on the
first attempt (manual boundary-value construction, the correct tool for this arithmetic/logic-bug
class per `HEURISTICS.md`), and the second approach (the standalone real-API repro) was built to
satisfy `CONFIRMATION.md`'s reproducibility bar with the project's actual public API rather than
to hunt for a new signal.

## Status

**Confirmed**. Reproduces 3/3 runs in a debug build (deterministic - pure floating-point
arithmetic, no seed/thread-dependence), and separately reproduces via the standalone `repro/`
crate built against the real `polycool` public API in a release build. See `output.txt` for the
actual captured output of both.

**Fully reproducible**: **Yes**. This is a single-participant finding: the "input" is just the
three `f64` coefficients passed directly to `Quadratic::new`, the same public constructor any real
caller (including `polycool`'s own `cubic.rs`) would use - there is no peer/caller relationship
being faked here, so `CONFIRMATION.md` section 11's two-participant bar does not apply. `repro/`
calls the real, unmodified public API (`Quadratic::new`, `.roots()`, `.positive_discriminant_roots()`)
against a real clone of the crate at the stated base commit.

## Class

logic

## Root cause

`positive_discriminant_roots` and `roots` both derive `q = -0.5 * (b + disc.sqrt().copysign(b))`,
`r0 = q / a`, `r1 = c / q` from the same three coefficients. When `a == 0.0` (the quadratic
degenerates to the linear equation `b*x + c == 0`), `disc = b*b` is still finite and positive
(assuming `b != 0`), so the `disc > 0.0` branch is taken in both functions - but `r0 = q / a`
divides by zero, producing `+-inf`. `roots()` accounts for this explicitly: its `push` closure
checks `r.is_finite()` before adding a value to the result, so the infinite `r0` is silently
dropped and only the finite root (`r1`, the genuine linear-equation solution) survives.
`positive_discriminant_roots` has no equivalent check - it unconditionally packs `r0` and `r1`
into the returned tuple via `r0.min(r1)`/`r0.max(r1)`, so the infinite value survives into the
`Some(...)` result exactly as the doc comment ("returns the two distinct roots") is violated: there
is only one real root here, not two, and one side of the returned pair is `-inf`/`inf` rather than
a real root at all.

The function's only current caller, `cubic.rs::roots_between_with_buffer` (line 174), happens to
mask this because it immediately checks the returned bounds against a finite `lower <= x <= upper`
window before using them - an infinite bound simply always fails that check. But
`positive_discriminant_roots` is a public method on a public type (`pub fn` on `Quadratic`, no
`#[doc(hidden)]`, no documented restriction against `a == 0`), so any other caller (in `polycool`
or a downstream crate) invoking it directly on a linear-degenerate quadratic gets a silently wrong
`Some((-inf, finite_root))` or `Some((finite_root, inf))` instead of either the correct single root
or a `None` reflecting "there aren't two roots here."

## Introduced by

0786080 "Bring poly-cool into the kurbo repo (#502)" (2025-10-18, PR #502) - this is the commit
that added `polycool/src/quadratic.rs` (and both `roots()` and `positive_discriminant_roots()`,
already in their current asymmetric-guarding form) into the `kurbo` repository. `git log -S` on
`positive_discriminant_roots` finds no earlier occurrence in this repo's history; the function may
have existed with the same shape in the standalone `poly-cool` project prior to the merge, but that
history is not part of this repo and wasn't checked.

## The fix

Added the same finiteness check `roots()` already uses, applied to both `r0` and `r1` before
constructing the `Some(...)` pair:

```rust
if disc > 0.0 {
    let q = -0.5 * (b + disc.sqrt().copysign(b));
    let r0 = q / a;
    let r1 = c / q;
    if r0.is_finite() && r1.is_finite() {
        Some((r0.min(r1), r0.max(r1)))
    } else {
        None
    }
} else {
    None
}
```

`None` was chosen (rather than trying to return the single surviving finite root somehow) because
the function's contract is specifically "the *two* distinct roots" - when only one root is real,
that contract can't be satisfied, and `None` is what the function already returns whenever there
aren't two roots to report (the `disc <= 0.0` branch). This matches `roots()`'s own treatment of
the same arithmetic (silently dropping the non-finite value) without changing
`positive_discriminant_roots`'s return type or its meaning for every well-conditioned input.

Verified against 3 inputs in the same bug class (not just the one that first triggered it, per
`CONFIRMATION.md` section 7): the original degenerate case (`2x + 1 = 0`), a second, differently-
coefficiented degenerate case (`3x - 6 = 0`), and a normal, non-degenerate quadratic
(`x^2 - 3x + 2 = 0`, unaffected, still returns `Some((1.0, 2.0))`) - see `output.txt`.

- Project test suite (`polycool` package, `cargo test -p polycool --release -- --skip
  planted_root_deg5`; `planted_root_deg5` skipped as a pre-existing, unrelated flaky test, see
  candidate `017.POLYCOOL_PLANTED_ROOT_ILL_CONDITIONED_TOLERANCE`): **9 passed, 0 failed before ->
  9 passed, 0 failed after** (no regressions, no new failures).
- Whole-workspace test suite (`cargo test --workspace --release -- --skip planted_root_deg5`):
  **185 passed, 2 failed before -> 185 passed, 2 failed after**, identical in both cases. The 2
  failures (`kurbo::bezpath::tests::test_elements_to_segments_starts_on_closepath` and
  `kurbo::bezpath::tests::test_must_not_start_on_quad`) were independently confirmed to be
  pre-existing on the unpatched base commit too (unrelated to `polycool`/`quadratic.rs`) - not
  caused or affected by this patch.

## What's not done / open questions

- `positive_discriminant_roots_scaled` (the `#[cold]` overflow-rescaling sibling, lines 100-108)
  calls back into the now-fixed `positive_discriminant_roots`, so it inherits the fix automatically
  - not a separate code path that needed its own patch.
- Did not check whether the standalone, pre-merge `poly-cool` project (if it still exists
  independently) has the same asymmetry, since only this repository's history was examined.
- No broader fuzzing/property-testing campaign was run against `positive_discriminant_roots`
  beyond the specific boundary values constructed by hand; a longer `arbtest`/fuzz campaign over
  arbitrary `a` near zero (not just exactly `0.0`) might turn up a related near-degenerate case
  where `r0`/`r1` are enormous-but-finite rather than infinite, which this fix would not address
  (the underlying numerical-conditioning question, as opposed to this literal "silently returns
  infinity" bug) was not investigated.
