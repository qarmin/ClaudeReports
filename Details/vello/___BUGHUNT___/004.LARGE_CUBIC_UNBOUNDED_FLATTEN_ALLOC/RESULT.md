# Large-magnitude cubic Bezier crashes during flattening (issue #1277)

`sparse_strips/vello_common/src/flatten_simd.rs`'s `flatten_cubic_simd` derives the number of
output line segments (`n`) for a cubic Bezier directly from an unbounded curvature/subdivision
estimate (`sum`), then immediately `Vec::resize`s the reusable flattening buffer to `n + 4`
`Point32` entries with no upper bound. For a cubic whose control points are extreme relative to
its endpoints, `sum` (and therefore `n`) can reach billions, so `resize` requests tens of
gigabytes and the process aborts on allocation failure. Issue #1277
(<https://github.com/linebender/vello/issues/1277>) gives a concrete failing unit test for this.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Ran the issue's literal repro (adapted only for the `fill()` API's added `cull_bbox: RectU16` parameter, added since the issue was filed) as a `#[cfg(test)]` in `vello_common::flatten` | debug build (`cargo test -p vello_common --lib`), then release | Does **not** crash on current main - the curve's every point has a y-coordinate far below any u16-representable cull-bbox top, so `flatten()`'s "fully outside the cull bbox" check discards the whole curve before `flatten_cubic_simd` is ever reached (see Root cause) |
| Manual boundary-value construction: same class of curve (endpoints in-view, control points at ~1e19), so the "fully outside" cull check does not fire and the curve reaches `flatten_cubic_simd` | debug and release builds, `cargo test -p vello_common --lib` | Deterministic `memory allocation of 56504737824 bytes failed` / `SIGABRT`, both build profiles |
| Repeated runs for the confirmation bar | release build, 3 runs via the standalone `repro/` crate (built from a fresh `clone` of the exact commit both times) | 3/3 aborts before the patch; 3/3 clean completions after the patch |

No `miri`/ASAN/fuzzing was needed - this is a plain unbounded-allocation-size computation on
floating-point input, not a memory-safety or UB question; manual boundary-value construction fully
characterized it on the second attempt (the first attempt, the issue's literal points, showed the
cull-bbox change added after the issue was filed incidentally routes around that exact input).

## Status

**Confirmed**. Reproduces 3/3 runs, release build (also reproduces in a debug build), fully
deterministic: the crash is not layout/timing/seed-dependent, it's a direct function of the
floating-point subdivision estimate for the given control points.

**Fully reproducible**: **Yes**. `repro/` calls the real, public `vello_common::flatten::fill`
entry point (the exact function every vello renderer backend calls to flatten a filled `BezPath`)
with a `BezPath`/`Affine`/`RectU16` a real caller would construct - it never calls the private
`flatten_cubic_simd` or any other internal function directly. This is a single-participant case
(the "attacker" is just document/path data, not a second process/peer), so `CONFIRMATION.md`
section 11's "single-process, single-participant repro remains completely fine" carve-out applies.

## Class

crash

## Root cause

In `flatten_cubic_simd` (`sparse_strips/vello_common/src/flatten_simd.rs`):

```rust
let sum: f32 = ctx.val[..n_quads].iter().sum();
let n = ((0.5 * sum / sqrt_tol).ceil() as usize).max(1);
let target_len = n + 4;
if target_len > ctx.flattened_cubics.len() {
    ctx.flattened_cubics.resize(target_len, Point32::default());
}
```

`n_quads` (the number of quadratics the cubic is first split into) *is* bounded, to `MAX_QUADS =
16` (`estimate_num_quads`/`estimate`). But `val[i]` (each quad's own subdivision-count estimate,
computed in `estimate_subdiv_simd`) is not bounded at all, and neither is their sum `n`, which is
used directly to size a heap allocation. For an ordinary curve this is a small, sane number. For a
curve where the control points overshoot the endpoints by many orders of magnitude, the estimated
curvature/subdivision requirement blows up, `n` reaches into the billions, and
`Vec::resize(n + 4, ..)` (8 bytes per `Point32`) requests tens of gigabytes, which fails and aborts
the process (`memory allocation of 56504737824 bytes failed`).

Separately, the exact point values from issue #1277 no longer reach this code at all on current
main: `flatten()` (in the same file) now takes a `cull_bbox: RectU16` (added by
PRs #1341/#1519/#1559, all landed after the issue was filed) and discards a cubic outright if all
four of its points are provably outside the bbox - including "all points have `y < top`". Every
point in the issue's literal curve has `y` in the `-1e18`..`-1e19` range, which is `< top` for any
u16-representable cull bbox (`top` is always `>= 0`), so the whole curve is always culled before
`flatten_cubic_simd` runs. This is incidental (the cull check exists for performance, not as a fix
for #1277) and does not hold once at least one endpoint is on-screen - see `repro/src/main.rs`'s
`on_screen_extreme_control_points()`, which triggers the identical crash through the identical
public entry point.

## Introduced by

9f4ecdea "Add support for SIMD flattening (#1092)" (2025-07-21) - found via `git log -S "let n =
((0.5 * sum / sqrt_tol).ceil() as usize).max(1);" -- sparse_strips/vello_common/src/flatten_simd.rs`,
which returns exactly this one commit (the line is unchanged from its introduction to `HEAD` other
than the `.max(1)` -> `.clamp(1, MAX_CUBIC_LINES)` change made by this session's patch). This
predates issue #1277 (filed 2025-10-15).

## The fix

Adds an upper bound, `MAX_CUBIC_LINES = 1 << 16` (65536), on the number of line segments a single
cubic is flattened into, and clamps `n` to it instead of only flooring it at 1:

```rust
let n = ((0.5 * sum / sqrt_tol).ceil() as usize).clamp(1, MAX_CUBIC_LINES);
```

This mirrors the precedent already in this file for `MAX_QUADS` ("This limit should not be hit in
normal operation, but _might_ be hit for very large transforms"): ordinary curves never come close
to 65536 segments, so behavior is unchanged for all realistic input, while pathological input now
trades flattening accuracy (a curve gets flattened more coarsely than the ideal tolerance would
otherwise produce) for never requesting an unbounded allocation. The rest of the function's math
(`step = sum / n`, the per-quad `dn`/`x0` bookkeeping) is scale-invariant with respect to `n`, so
capping it does not require any other change.

Verified against the standalone `repro/`:
- Before the patch: aborts with `memory allocation of 56504737824 bytes failed` / `SIGABRT`,
  reproduced 3/3 runs (release build; also 1/1 in a debug build during initial triage).
- After the patch: completes normally, printing `produced 65537 lines` then `Done: no crash.`,
  reproduced 3/3 runs. `65537 = MAX_CUBIC_LINES + 1`, i.e. the cap is what's binding here.

See `output.txt` for the actual captured before/after terminal output.

- Project test suite:
  - `cargo test -p vello_common --lib`: **159 passed / 0 failed / 1 ignored**, identical before
    and after the patch.
  - `cargo test -p vello_cpu --lib`: **81 passed / 0 failed**, identical before and after the
    patch.

## What's not done / open questions

- The analogous unbounded `n` in the `QuadTo` branch of `flatten()` (same file,
  `let n = ((0.5 / SQRT_TOL * params.val).ceil() as usize).max(1);`) was not checked - a quadratic
  Bezier with an extreme control point could plausibly hit the same class of unbounded-allocation
  bug through a different code path (this one allocates a stack-sized loop rather than a `Vec`
  resize, via `for i in 1..n`, so the failure mode there would more likely be a long-running loop /
  hang than an OOM abort, but it wasn't exercised this session).
- `kurbo`'s own reference `flatten`/`CubicBez::to_quads` implementation (vendored dependency, not
  part of this repo) has the identical unbounded-`n` shape and was not checked for whether the same
  input crashes a plain `kurbo`-only caller outside of vello.
- Whether `vello_hybrid`'s or `vello_shaders`' own (separate) flattening code paths
  (`vello_shaders/src/cpu/flatten.rs`, `vello_shaders/shader/flatten.wgsl`) share the same
  unbounded-estimate pattern was not checked - `IDEAS.md` already has a related, broader entry for
  transform-scale-dependent subdivision hangs (issue #1752) that may be worth cross-referencing.
- The chosen cap (`1 << 16`) is a judgment call, not derived from a specific accuracy/memory
  budget analysis; a maintainer may want a different value or to make it configurable.
