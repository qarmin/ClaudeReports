# Transform-scale-dependent subdivision loop hang on extreme zoom (issue #1752)

`sparse_strips/vello_common/src/flatten_simd.rs`'s `flatten` function derives the number of output
line segments for a `PathEl::QuadTo` (quadratic Bézier) directly from an affine-transform-scale-
dependent curvature/subdivision estimate with no upper bound, then runs a plain `for i in 1..n`
loop, one iteration per output line, with no allocation up front that could fail and abort the
process. At a large enough "zoom" transform scale, `n` grows into the billions-to-quintillions and
the loop simply keeps running - a genuine hang, not a crash. Font glyph outlines are commonly made
of quadratic Béziers, which is consistent with the issue's "nearly any demo scene" (most draw
text) description. This is the same class of bug as this workspace's own
`004.LARGE_CUBIC_UNBOUNDED_FLATTEN_ALLOC` candidate (issue #1277, the analogous unbounded estimate
on the cubic/`CurveTo` path), which that candidate's own "What's not done" section flagged as an
open lead pointing at exactly this `QuadTo` code and exactly this issue (#1752).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: computed `QuadBez::estimate_subdiv`'s output (`val`, hence the loop count `n`) directly for an ordinary-shaped quad curve scaled by `10^exp` for `exp` in `{0,2,...,300}`, via a temporary `#[ignore]`d unit test added to `flatten_simd.rs` (removed before finalizing) | debug build, `cargo test -p vello_common --lib -- --ignored --nocapture` | Confirmed the mechanism analytically: `val` grows roughly as `sqrt(scale)` for finite `scale`; at `scale=1e30`, `n ≈ 1.23e15`; at `scale>=1e50`, `val` is still a *finite* f64 but the `as usize` cast on it saturates to `usize::MAX` (`18446744073709551615`) - only past `scale≈1e200` does `val` itself become non-finite (in which case the code's own `is_finite` guard resets it to `0`, incidentally avoiding the hang at truly astronomical scales) |
| Real end-to-end timing measurement: called the actual public `vello_common::flatten::fill` entry point with an ordinary `QuadTo`-only `BezPath` and `Affine::scale(10^exp)`, release build (`opt-level=3`), measuring wall-clock time via `std::time::Instant` | release build, `cargo test -p vello_common --lib ... --release -- --ignored --nocapture`, then the standalone `repro/` crate | Elapsed time grows linearly with the resulting line count `n`, exactly as the loop-count analysis predicts: `scale=1e10` -> 123,006 lines / ~1.5ms; `scale=1e12` -> 1,230,049 lines / ~15ms; `scale=1e14` -> 12,300,473 lines / ~140ms; `scale=1e16` -> 123,004,719 lines / ~1.37s. At `scale=1e40`, `n` has saturated to `usize::MAX` and the same call does not return within a 5s timeout (nor a follow-up 2s retry) - reproduced identically 3/3 runs |
| Same measurement repeated against a normal-looking `CurveTo` (cubic) curve, not an "extreme control points" case like candidate 004's - just uniformly scaled | release build, same harness | Same growth pattern (`scale=1e16` -> 178,315,521 lines / ~1.6s), confirming the underlying mechanism (an unbounded, transform-scale-dependent subdivision estimate with no cap) is shared between the `QuadTo` and `CurveTo` code paths, though the `CurveTo` path additionally does an up-front `Vec::resize` that can abort on allocation failure before ever reaching the compute loop (candidate 004's finding) - this candidate's fix and repro deliberately target only the `QuadTo` path, which has no such allocation step and therefore hangs rather than crashes at any scale |

No `miri`/ASAN/fuzzing was needed - this is a plain unbounded-loop-count computation on
floating-point input tied directly to affine transform scale, not a memory-safety or UB question;
manual boundary-value construction plus a real timed run of the public API fully characterized it.

## Status

**Confirmed**. Reproduces 3/3 runs, release build, fully deterministic (not layout/timing/seed-
dependent - the loop count is a direct, reproducible function of the transform scale and curve
shape). At `scale=1e40` the call to `fill()` does not return within a 5-second timeout on every
run; at `scale>=1e50` the loop count analytically saturates to `usize::MAX`, which at the measured
throughput (~90M lines/second in this environment, extrapolated from the `scale=1e16` measurement
below) would take on the order of 6500 years of continuous CPU time to complete - a hang for any
practical purpose, not merely "slow."

**Fully reproducible**: **Yes**. `repro/` calls the real, public `vello_common::flatten::fill`
entry point (the same function every vello renderer backend calls, via
`StripGenerator::generate_filled_path`, to flatten any filled path including glyph outlines) with
an entirely ordinary `QuadTo` curve and a plain `Affine::scale(...)` transform - exactly what
"zooming in" applies to every path in a scene. This is a single-participant case (the "attacker"
is just an unusually large zoom level, not a second process/peer), so `CONFIRMATION.md` section
11's "single-process, single-participant repro remains completely fine" carve-out applies, the
same as candidate 004.

## Class

crash

<!-- "crash" is used here in the same broad sense the workspace's Status/Class legend intends for
a process that stops behaving usefully; more precisely this is a hang (the process never returns
control, but also never panics/aborts) rather than a crash/panic/abort. -->

## Root cause

In `flatten` (`sparse_strips/vello_common/src/flatten_simd.rs`), the `PathEl::QuadTo` arm:

```rust
let q = QuadBez::new(p0, p1, p2);
let params = q.estimate_subdiv(SQRT_TOL);
let n = ((0.5 / SQRT_TOL * params.val).ceil() as usize).max(1);
let step = 1.0 / (n as f64);
for i in 1..n {
    let u = (i as f64) * step;
    let t = q.determine_subdiv_t(&params, u);
    let p = q.eval(t);
    callback.callback(LinePathEl::LineTo(p));
}
```

`params.val` (from `QuadBez::estimate_subdiv`) is a curvature/arc-length-derived estimate of how
many line segments are needed to approximate the curve within a fixed *absolute* tolerance
(`SQRT_TOL = 0.5`, i.e. half a pixel). Because `p0`/`p1`/`p2` are already transformed by the
caller's `affine` (see `flatten`'s `match affine * el`), zooming in on a scene scales these points
up, and the physical arc length of the curve (in the transformed, pixel-ish coordinate space) grows
proportionally - `val`'s dominant term (`scale.sqrt()` in the curvature formula, where `scale`
itself grows roughly linearly with the transform's scale factor for a fixed-shape curve) therefore
grows like `sqrt(transform_scale)`. Since the output tolerance stays fixed at half a pixel, more
and more line segments are genuinely required to keep the flattened polyline within tolerance of
the (now enormous) true curve - and nothing caps how many. `n` is used directly as a loop bound
with **no upper bound analogous to `MAX_QUADS`** (which caps the *cubic*-to-quad split count, a
different variable), so:

- At moderate-but-real zoom levels (`transform_scale` around `1e14`-`1e16`, reachable by a user
  zooming in a few dozen to a few hundred steps), `n` reaches the tens of millions to low hundreds
  of millions, and the loop - real, non-vectorized work per iteration (`determine_subdiv_t`,
  `eval`, a callback call per line) - takes on the order of 0.1-1.5 seconds *per curve*, in a
  render path that runs synchronously on every frame; a scene with more than a handful of such
  curves (e.g. any text) stalls badly.
- At more extreme zoom (`transform_scale >= 1e50`), the `f64` value of `params.val` is still
  finite (arithmetic hasn't overflowed to infinity), but is large enough that `as usize` saturates
  to `usize::MAX`. The `for i in 1..n` loop then has no way to ever finish in practice - this is
  the literal "never terminates" hang the candidate description named.

Unlike the cubic (`CurveTo`) path (`flatten_cubic_simd`, candidate 004's finding), there is no
`Vec::resize`/allocation step here that could fail and abort the process for extreme `n` - the
loop just runs directly, which is why this manifests specifically as a **hang**, not a crash, and
is the more literal match for issue #1752's "hangs the app" / "never terminates" description.

## Introduced by

9f4ecdea "Add support for SIMD flattening (#1092)" (2025-07-21) - found via `git log -S 'let q =
QuadBez::new(p0, p1, p2);' --all -- sparse_strips/vello_common/src/flatten_simd.rs
sparse_strips/vello_common/src/flatten.rs`, which returns exactly this one commit. This is the same
commit candidate 004 identified as introducing the analogous unbounded cubic estimate - this single
commit introduced both unbounded estimates when it rewrote path flattening to use SIMD. Before this
commit, `PathEl::QuadTo` was not handled at all in the flattening code (`unreachable!()` in the
original `vello_common` commit, 7a8d8f39, since paths were presumably normalized to cubics
upstream at the time).

## The fix

Adds an upper bound, `MAX_QUAD_LINES = 1 << 16` (65536), on the number of line segments a single
`QuadTo` curve is flattened into, mirroring the naming/shape of candidate 004's `MAX_CUBIC_LINES`
fix for the cubic path:

```rust
let n = ((0.5 / SQRT_TOL * params.val).ceil() as usize)
    .clamp(1, MAX_QUAD_LINES);
```

Ordinary curves never come close to 65536 segments (the moderate-zoom measurements above show
even a real, plausible zoom level - `1e16` - would otherwise need ~123 million), so behavior is
unchanged for realistic input; pathological/extreme-zoom input now trades flattening accuracy
(coarser output than the ideal tolerance) for bounded, near-instant completion instead of an
unbounded hang. `step`/`determine_subdiv_t`'s math does not depend on `n` being "correct" in any
way other than being a loop bound, so capping it requires no other change.

Verified against the standalone `repro/` (3/3 runs each, release build):
- Before the patch: `scale=1e10..1e16` complete but take up to ~1.37s; `scale=1e40` times out
  after 5s (and again after a 2s retry) on every run.
- After the patch: every scale, including `1e40`, completes in well under 1 millisecond, always
  producing exactly `65537` lines (`= MAX_QUAD_LINES + 1`, confirming the cap is what's binding).

See `output.txt` for the actual captured `just test` output (fresh clone, before-patch run,
patch applied, after-patch run, all in one real invocation).

- Project test suite:
  - `cargo test -p vello_common --lib`: **159 passed / 0 failed / 1 ignored**, identical before
    and after the patch.
  - `cargo test -p vello_cpu --lib`: **81 passed / 0 failed**, identical before and after the
    patch.

## What's not done / open questions

- This candidate deliberately fixes only the `QuadTo` path. The `CurveTo` (cubic) path's own
  unbounded estimate is candidate 004's finding (a separate, already-recorded candidate with its
  own patch, for the allocation-failure/abort flavor of this same underlying issue class) - not
  duplicated here per this workspace's "one bug per candidate folder" rule. A maintainer applying
  both patches would close out both the "abort on extreme relative control points" and "hang on
  merely being zoomed in a lot" manifestations of issue #1752/#1277.
- The exact zoom level a real user needs to reach to hit the "several-second stall" regime
  (`~1e14`-`1e16`) versus the outright multi-minute-to-effectively-permanent hang regime
  (`>=1e20`-`1e50`) depends on the specific curve's shape and the demo/viewer's own zoom-step
  size; this session measured the underlying `fill()` mechanism directly rather than driving an
  actual windowed demo app's mouse-wheel zoom handler end-to-end.
- Whether `vello_hybrid`'s GPU-side flattening (`vello_shaders/src/cpu/flatten.rs`,
  `vello_shaders/shader/flatten.wgsl`) shares an analogous unbounded-estimate pattern was not
  checked this session.
- `kurbo`'s own reference `estimate_subdiv`/`flatten` implementation (vendored dependency, not
  part of this repo) has the same unbounded-estimate shape in its own public API and was not
  checked for whether the same input hangs a plain `kurbo`-only caller outside of vello.
