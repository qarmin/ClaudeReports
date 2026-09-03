# SOLVE_ITP_FALLIBLE_SHIFT_OVERFLOW

`common::solve_itp_fallible` (kurbo/src/common.rs:713, in the released 0.13.1 tree) builds its ITP
step schedule as `epsilon * (1u64 << nmax) as f64`, where `nmax` is derived from `epsilon` with no
upper bound. `ParamCurveArclen::inv_arclen` derives `epsilon = accuracy / total_arclen`, so a long
enough curve drives `epsilon` small enough that `nmax` reaches or exceeds 64. This candidate came
from a named suspect (linebender/kurbo#602) with two competing open, unmerged fix PRs (#588, #600)
already proposing different-sized fixes for it.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction, `cargo run --example` in a scratch worktree, debug-assertions build (default `dev` profile) | `CubicBez::new((0,0),(5e7,0),(5e7,1e7),(1e8,1e7))`, `accuracy = 1e-6` (candidate's own suggested repro) | Did not panic/hang - `nmax` only reaches ~47-54 for this curve size, under the 64-bit shift width |
| Same, scaled 100x (`5e9`/`1e10` control points) | `accuracy = 1e-6` | Still did not panic/hang - `nmax` ~54, still under 64 |
| Read linebender/kurbo#602 (`gh api repos/linebender/kurbo/issues/602`) for the maintainer-confirmed exact repro | `CubicBez::new((0,0),(1e12,0),(0,0),(1,0))`, `accuracy = 1e-9` | Panicked reliably in debug ("attempt to shift left with overflow" at common.rs:713), hung in release (never returned inside an 8s `timeout`) |
| Read open PR #588 (narrow fix: `1u64 << nmax` -> `2.0_f64.powi(nmax)`) and PR #600 (broader fix: float-based schedule + clamped `r` + endpoint-rounding termination), applied #588 alone to the scratch worktree and re-ran both the #602 repro and #600's own reported repro (`CubicBez::new((0,0),(5e7,0),(5e7,1e7),(1e8,1e7))`, `arclen(1e-9) * 0.171`, `accuracy = 1e-9`) | Debug + release, `cargo run` / `cargo run --release` | #588 alone fixed the #602 repro (shift overflow) in both builds, but did **not** fix #600's repro - that one still hung under `timeout` in release with #588 applied, confirming it is an independent bug ("bug one" in #600's write-up: the ITP bracket can stop shrinking once it is narrower than adjacent-`f64` spacing, with no shift overflow involved at all - `nmax` stays under 64 for this curve) |

## Status

**Confirmed**. Reproduces deterministically 3/3 runs for both sub-cases, same build, same input,
default environment (see `output.txt` / `repro/` for the actual captured runs, which additionally
re-confirm this against a completely fresh clone before/after the fix).

Two genuinely distinct bugs share the one symptom (`inv_arclen` never returns a usable answer on a
long-enough curve):

- **Shift overflow** (what the candidate name names): `nmax >= 64` makes `1u64 << nmax` overflow -
  panics in a debug/overflow-checked build, silently wraps to `nmax & 63` in release (shift amount
  masked to 6 bits on x86_64), producing a `scaled_epsilon` that is `2^64` times too small, so the
  ITP loop's `r` term is negative on every pass and the bracket never shrinks below `2 * epsilon` -
  an unbounded spin.
- **Float-resolution non-termination** (independent, does not require `nmax >= 64` at all): once
  the tolerance `epsilon` the caller asks for is finer than the spacing between adjacent `f64`
  values near the root, the bracket `[a, b]` can shrink until `a`/`b` are the two nearest
  representable floats and go no further, but the `while b - a > 2.0 * epsilon` exit condition
  never becomes true, and the interpolated point keeps rounding exactly onto one of the (unchanged)
  endpoints, so `a = xitp`/`b = xitp` become no-ops and the loop repeats the same point forever.

**Fully reproducible**: **Yes**. `repro/` calls kurbo's real public `ParamCurveArclen::inv_arclen`
(via `CubicBez`, the project's own public API) with two concrete curves that are themselves the
complete external input - there is no second participant being faked here (CONFIRMATION.md section
11's exception for a single external artifact with no "other side" applies directly: a `CubicBez`
constructed and handed to a public method is exactly what any real caller does).

## Class

crash / hang (panics under debug-assertions, spins forever consuming 100% CPU with no panic and no
error return under release - matches linebender/kurbo#602's own description, including the WASM
"unkillable spin" consequence noted there).

## Root cause

`solve_itp_fallible` (kurbo/src/common.rs, `pub(crate) fn solve_itp_fallible`) computes:

```rust
let n1_2 = (((b - a) / epsilon).log2().ceil() - 1.0).max(0.0) as usize;
let nmax = n0 + n1_2;
let mut scaled_epsilon = epsilon * (1u64 << nmax) as f64;
```

`ParamCurveArclen::inv_arclen` (kurbo/src/param_curve.rs) calls this with `a = 0.0`, `b = 1.0`,
`epsilon = accuracy / total_arclen` - a tolerance on the curve *parameter* `t`, derived by dividing
the caller's arc-length `accuracy` by the curve's total arc length. On a long curve this `epsilon`
is tiny, which two different parts of `solve_itp_fallible` mishandle:

1. `nmax = n0 + ceil(log2((b-a)/epsilon)) - 1` grows without bound as `epsilon` shrinks. Once it
   reaches 64, `1u64 << nmax` is undefined-shift-amount UB territory in C terms and, in Rust,
   panics with overflow checks on (`attempt to shift left with overflow`) or silently computes
   `1u64 << (nmax % 64)` with checks off - both wrong, and the release case is actively worse
   (produces a plausible-looking but incorrect `scaled_epsilon` rather than failing loudly).
2. Independently of (1), the loop's exit condition `while b - a > 2.0 * epsilon` and its
   interpolation step assume `epsilon` is coarser than the `f64` resolution of the bracket. When it
   isn't, `xitp` rounds onto `a` or `b` (whichever `x1_2`/`xf`/`xt` round to), the corresponding
   assignment (`a = xitp` or `b = xitp`) is then a self-assignment, `b - a` stops changing, and the
   loop condition stays true forever with no protection against it.

## Introduced by

08962c1eaa18235848881ced7dad9db3060febfd "Fancier solver for inverse arclength" (2021-03-12) - the
commit that introduced the ITP-based solver and the `1u64 << nmax` schedule construction
(`git log --oneline --all -S'1u64 << nmax'` and `-S'scaled_epsilon'` both point here as the first
commit containing this pattern; a later refactor, 021e291 "Deduplicate `solve_itp` functions"
(#560), moved the code into its current `solve_itp`/`solve_itp_fallible` split but preserved the
same shift and the same missing termination guarantee verbatim).

## The fix

Applied directly to `kurbo/src/common.rs`, `solve_itp_fallible`, in a scratch worktree at
ca273499e3e48bd2de6f02aa8e99a148984e45f3 (see `patch.diff`):

- Replace `epsilon * (1u64 << nmax) as f64` with `epsilon * 2.0_f64.powi(powi_nmax)` (`nmax` clamped
  into `i32` range first). `f64::powi` is already available in both the `std` and the `no_std`/
  `libm` build (it's already in kurbo's own `define_float_funcs!` shim, used elsewhere in this same
  file), so this needed no new float-function shim, unlike an `f64::exp2()`-based version (tried
  first, reverted - `exp2` is not in `FloatFuncs` and broke `cargo build -p kurbo --no-default-features
  --features libm`, caught by actually building that configuration, not just `std`). `powi`
  saturates to `f64::INFINITY` rather than panicking/wrapping once `nmax` is large enough, which
  only feeds into the `r` clamp below rather than causing new incorrect behavior.
- Clamp `r` (`scaled_epsilon - 0.5 * (b - a)`) to a minimum of `0.0`, matching the ITP paper's own
  invariant that this quantity is never negative - unclamped, it goes negative once the bracket
  falls behind the `nmax` schedule (which the old code could only reach via the shift overflow, but
  the clamp is needed unconditionally since the schedule can also legitimately fall behind for
  smaller, non-overflowing `nmax`).
- After computing `xitp`, fall back to the bisection midpoint `x1_2` if `xitp` didn't land strictly
  inside `(a, b)` (can happen once `epsilon` is finer than the local `f64` resolution even with no
  overflow at all - this is what fixes the second, independent bug), and if even the midpoint can't
  land strictly inside `(a, b)`, return the current bracket immediately instead of continuing the
  loop - at that point every representable value has been exhausted between `a` and `b`, so no
  further iteration can ever make progress.

This fixes the same two sub-bugs PR #600 fixes, using the same mechanism for the termination issue
(the `r` clamp and the endpoint-rounding bisect-then-stop check) but PR #588's narrower, no-new-shim
approach for the schedule construction itself (`powi` rather than `exp2`) - verified here
independently rather than by taking either PR's patch on faith: the two sub-bugs and the "PR #588
alone is insufficient" finding were established by first testing PR #588 in isolation (see the table
above), and the `exp2`-breaks-`no_std` gap was caught by actually building the `libm` feature
configuration, not assumed from reading either PR's diff.

Verified against `repro/`'s two cases, both build profiles, before and after:

- Before: case A panics in debug, hangs (release) - reproduced 3/3 manual runs plus the `just test`
  run captured in `output.txt`. Case B hangs in both debug and release - reproduced 3/3 manual runs
  plus `output.txt`.
- After: both cases return a finite `t` in `(0, 1)` within the same call, in both build profiles,
  in `output.txt`'s "AFTER PATCH" sections and via `just test` against a fresh clone.
- Varied beyond the exact triggering inputs per CONFIRMATION.md section 7: confirmed the fix also
  resolves the original candidate-description repro
  (`CubicBez::new((0,0),(5e7,0),(5e7,1e7),(1e8,1e7))` at `accuracy = 1e-6`, which turned out not to
  actually trigger either bug at that accuracy - see "What was tried" - `1e-9` was needed) and the
  100x-longer variant, both continuing to return a sane `t` after the fix.
- Project test suite (`cargo test --workspace`, from the workspace root, `kurbo` + `polycool` +
  doctests): **before** patch: 187 + 10 + 55 passed, 0 failed, 1 ignored (a `#[ignore]`d doctest in
  `polycool`, unrelated). **After** patch: identical - 187 + 10 + 55 passed, 0 failed, 1 ignored. No
  new failures, no fewer passes.
- `cargo build -p kurbo --no-default-features --features libm` (the `no_std`-with-`libm` float-ops
  configuration): builds cleanly after the patch (this is what caught the `exp2`-based first draft
  of this fix as broken - see "The fix" above - and is why the final patch uses `powi` instead).

## What's not done / open questions

- `solve_itp` (the public, infallible wrapper around `solve_itp_fallible`) is fixed by the same
  patch, since it delegates directly to `solve_itp_fallible` - not verified with a separate repro
  since it has no separate code path, but worth a maintainer's own test if one doesn't already
  exist for it (PR #588 adds one; PR #600 adds three, exercising both sub-bugs plus `epsilon = 0.0`
  directly against `solve_itp`/`solve_itp_fallible` rather than through `inv_arclen`).
- Not fixed here: `epsilon == 0.0` (or `epsilon < 0.0`) passed directly to the public `solve_itp`
  (bypassing `inv_arclen`, which never produces such an `epsilon` itself) still overflows the
  `n0 + n1_2` addition in debug, since `n1_2`/`nmax` remain `usize` in this patch (`(b - a) / 0.0`
  is `inf`, `inf as usize` saturates to `usize::MAX`, and `n0 + usize::MAX` overflows the checked
  addition) - PR #600's write-up identifies this as a separate pre-existing bug and its own
  (larger, `f64`-based-`nmax`) fix happens to resolve it too, but it is out of scope for candidate
  007's named symptom (the shift overflow, which this patch does fix) and was not independently
  reproduced or fixed this session - left for a maintainer to decide whether it's worth a dedicated
  candidate of its own.
- `fit_to_bezpath`/`fit_to_bezpath_opt` (mentioned in both #602 and #600 as the real-world caller
  that surfaces this through curve fitting rather than a direct `inv_arclen` call) was not
  independently exercised here - `inv_arclen` is called directly, which is the actual affected
  entry point the bug lives in; the fitting code is simply one caller of it among others
  (`fit.rs`, `stroke.rs`'s dashing) that would all benefit equally from this fix.
- PR #600 also changes public-facing doc comments (`inv_arclen`'s accuracy-units documentation,
  `solve_itp`'s dropped `epsilon > 2^-63 * (b-a)` lower-bound requirement). This candidate's
  `patch.diff` intentionally omits those doc changes to keep the verified fix minimal and scoped
  to the confirmed defect; a maintainer merging either open PR would reasonably want the doc
  update too, since the `2^-63` lower bound this patch removes the practical need for is still
  stated as a hard requirement in `solve_itp`'s current doc comment.
