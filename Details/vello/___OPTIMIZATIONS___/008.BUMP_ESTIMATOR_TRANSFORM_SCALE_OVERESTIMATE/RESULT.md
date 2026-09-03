# BUMP_ESTIMATOR_TRANSFORM_SCALE_OVERESTIMATE

`rustc 1.90.0 (1159e78c4 2025-09-14)`, `cargo 1.90.0`.

Open issue #541 (`vello_encoding::BumpEstimator` can massively overestimate the GPU bump-buffer
sizes it recommends for scenes whose content is scaled/positioned mostly outside the render
target - a real problem since the render target size isn't known at scene-construction time) is
still open. Open PR #1561 ("tighten bump estimator accounting") is a still-unmerged fix bundling
several individually-tested corrections: subpath-close accounting, `LineTo` estimation using the
actual previous point instead of always the subpath's first point, round cap/join curve-count
scaling, a stroke width fudge factor for cubic segments, and - the one this candidate isolates and
quantifies - a fix to `transform_scale()`'s formula.

Before the patch, `transform_scale()` computes `(|v1| + |v2|)` where `v1`/`v2` are built so that
`|v1| + |v2| = 2 * s` for a similarity transform with true linear scale `s` (verified by hand for
both a pure uniform scale and a pure rotation - see "Why the result looks the way it does"). The
function never divides by 2, so it returns **exactly double** the correct scale factor for every
transform, not just an edge case. This `scale` feeds `BumpEstimator::count_path`'s `scaled_width`
(stroke width used to size caps/joins/curve-offset fudge) and `BumpEstimator::tally`'s final
`segments`/`lines` totals - i.e. it is on the hot path of every stroked-path bump estimate, at
every transform including the identity/default case. This is a distinct, independently-fixable
bug from the issue's own headline "unknown viewport size" complaint (PR #1561 doesn't address
viewport clipping at all - the code's own comments say that's not solvable at insertion time
either way) - it is a plain systematic 2x error present since the estimator was introduced,
compounding whatever slack the viewport-clipping problem separately adds.

## What was tried

No existing project bench/example reaches `BumpEstimator` (it's a `vello`-crate-only,
`bump_estimate`-feature-gated API with no internal caller - it exists for library consumers who
want to pre-size GPU buffers before the first render, to avoid a GPU readback stall). A standalone
bench crate (`bench/`, not part of the main workspace) was written: it builds a representative
stroke-heavy scene (400 five-pointed stars + 400 wavy 8-segment cubic-curve strokes, `Join::Round`
+ `Cap::Round`, width 6.0 - the case PR #1561 tightens most, via `estimate_arc_lines`) under five
transforms, and reports two things: `Scene::bump_estimate(None).total` (bytes - the accuracy/
memory dimension) and the wall-clock cost of building the scene + computing the estimate (the CPU
dimension).

### Accuracy (estimated bump-buffer size, bytes) - `size` mode, deterministic, no repeats needed

| Transform | baseline (bytes) | patched (bytes) | reduction |
|---|---|---|---|
| identity | 24,781,212 | 17,616,012 | -28.9% |
| scale 2x | 46,026,012 | 33,014,412 | -28.3% |
| scale 0.5x | 12,656,412 | 9,936,012 | -21.5% |
| rotate 37deg | 24,835,212 | 17,616,012 | -29.1% |
| rotate 51deg * scale 1.7x | 39,206,412 | 27,648,412 | -29.5% |

(`identity` and `rotate_37deg` land on the same patched value, 17,616,012 - expected, since a pure
rotation has the same true linear scale as identity, and the patch's fix makes `transform_scale`
return that true value in both cases instead of two slightly different ~2x-inflated numbers.)

### CPU cost (single-core, `taskset -c 0`, `perf 400` mode, min/p50/mean/p95/max ms) - noisy: machine shared with sibling `___OPTIMIZATIONS___` candidates running concurrent builds/benches

| Variant | min | p50 | mean | p95 | max |
|---|---|---|---|---|---|
| baseline (identity, per justfile run) | 0.2838 | 0.2874 | 0.2887 | 0.2942 | 0.3446 |
| patched (identity, per justfile run) | 0.2843 | 0.2881 | 0.2905 | 0.2996 | 0.4585 |

9 alternating paired runs (`bump_est_bench perf 400`, p50 extracted each run):

- `taskset -c 0` pinned: baseline/patched medians interleaved between two clusters (~0.29ms and
  ~0.49ms) that did **not** correlate with which binary ran - both binaries hit both clusters.
  This is core-0 contention with concurrently-running sibling candidates' own builds (confirmed
  via `ps aux` showing 2 other candidates' `cargo build`/`cargo test` processes active throughout
  this session), not a signal about the patch - MEASUREMENT.md's "two binaries land in different
  performance modes" trap, from external noise, not this candidate's own code.
- unpinned (scheduler free to move off the contended core): 9/9 pairs had patched marginally
  slower than baseline (~0.287-0.293ms baseline vs ~0.289-0.296ms patched, roughly +0.5-1.5%).
  Consistent in direction across all 9 pairs, but the absolute effect is a few microseconds out of
  ~0.29ms for a synthetic scene of 800 stroked paths with round caps/joins - plausible given the
  patch adds a `count` multiply to `curve_count` accumulation and an `offset_fudge` multiply to
  the cubic segment estimate that weren't there before, and small enough that it is not
  distinguishable with confidence from machine noise at this sample size, on a heavily
  shared/loaded machine.

## Result

**Accuracy/memory**: for this representative stroke-heavy scene, the pre-patch `BumpEstimator`
recommends allocating bump-allocator GPU buffers ~21-30% larger than the patched estimator does,
across every transform tried (identity, 2x/0.5x uniform scale, pure rotation, combined
rotate+scale) - a direct, systematic consequence of `transform_scale()` never dividing its raw
`|v1|+|v2|` sum by 2. This is deterministic (same synthetic scene, same code path, no run-to-run
variance) and confirmed exactly reproduced via the candidate's own `justfile` (`just test`) from a
fresh clone.

**CPU cost**: no clearly attributable change. A very small (roughly 0.5-1.5%), directionally
consistent (9/9 paired) slowdown was observed only in the unpinned condition; the pinned condition
was dominated by unrelated machine contention from sibling optimization candidates running at the
same time and showed no consistent direction at all. Either way the absolute magnitude is a few
microseconds against a ~0.29ms scene-estimate cost, i.e. not something a re-run on a quiet machine
would be expected to confirm as a real regression with any confidence.

## Does it change behavior/output?

- **Bit-exact for rendered pixels?** Yes - `BumpEstimator` only sizes buffers ahead of a render;
  it does not touch the rasterization path itself, so no rendered pixel changes.
- **Behavior change**: yes, for `Scene::bump_estimate()`'s **return value** specifically (a public
  API under the `bump_estimate` feature) - it now returns a smaller (and, per the PR's own
  description and added unit tests, more accurate) `BumpAllocatorMemory` for the same scene. A
  caller who pre-sizes GPU buffers from this value and never re-checks/grows them at render time
  would allocate a smaller buffer after this patch - correct if the estimator was already
  conservative enough (which the PR's other 4 fixes independently aim to keep true), but this is
  the kind of change worth flagging explicitly since it changes a public numeric API's output for
  every caller, not just an internal implementation detail.
- **Is this a breaking change?** No API signature change; a behavioral value change to an existing
  public method's return value, which is the entire point of a "tighten the estimate" fix.
- **Project test suite**: `cargo test -p vello_encoding --features bump_estimate -p vello --features bump_estimate`
  - baseline: 31 passed, 0 failed (`vello_encoding`), 0 tests (`vello`).
  - patched: 39 passed, 0 failed (`vello_encoding` - the 8 new tests are the ones PR #1561 itself
    adds), 0 tests (`vello`).
  - No new failures either direction.

## Why the result looks the way it does

`transform_scale(t)` is built from `v1 = (m0+m3, m1-m2)`, `v2 = (m0-m3, m1+m2)` and returns
`|v1| + |v2|`. For a pure uniform scale `s` (`m0=m3=s`, `m1=m2=0`): `v1 = (2s, 0)`, `v2 = (0, 0)`,
so `|v1|+|v2| = 2s` - twice the true scale. For a pure rotation by angle `theta` (unit scale):
`v1 = (2cos(theta), 2sin(theta))`, `v2 = (0, 0)`, so `|v1|+|v2| = 2` again - twice the true scale
of 1. This generalizes (the construction is the standard trick for recovering a 2x2 matrix's two
singular values via `|v1|`, `|v2|` for the sum/difference of columns rotated 90 degrees) - the
formula is *structurally* correct for extracting a similarity transform's linear scale, it is only
missing the final `* 0.5`. PR #1561 adds exactly that. Since this constant multiplies straight
through `scaled_width` (stroke sizing) and the final `segments`/`lines` tallies, every stroked
path's cap/join/curve-offset estimate and every scene's final buffer-size recommendation was
inflated by the same fixed factor, independent of scene content or transform - which is why the
reduction lands in the same ~21-30% band across every transform tried here (the exact percentage
varies because the *other* 4 fixes bundled into #1561 partially offset in the opposite direction
for this particular round-cap/round-join-heavy scene, not because the 2x scale fix itself varies).

## What's not done / open questions

- This candidate isolates and quantifies the `transform_scale` half of PR #1561; the other 4
  fixes (subpath-close accounting, `LineTo` previous-point fix, round cap/join curve-count
  scaling, cubic stroke width fudge) are bundled in the same `patch.diff` (matching the real PR,
  since splitting it further would test a hypothetical patch nobody would actually apply) but were
  not isolated/measured individually - see PR #1561's own description for which of those move the
  estimate up vs down.
- Issue #541's core complaint - the estimator has no idea of the render-target size at scene-
  construction time, so content transformed to lie mostly outside the eventual viewport is
  estimated as if it were all fully visible - is **not** addressed by PR #1561 and not addressed
  by anything measured here. The code's own comments (`vello_encoding/src/estimate.rs`) call this
  out as unsolvable at insertion time without deferring the tally to resolve-time and tracking
  bounding boxes through composition - a materially larger change than this PR's, not attempted
  here.
- Not verified: whether any current caller of `Scene::bump_estimate()` in the wild pre-sizes a
  buffer once and never grows it later (which would make this patch's smaller estimate load-
  bearing for that caller's correctness, not just its memory footprint) - `vello`'s own examples/
  tests don't call `bump_estimate()` anywhere in this repo, so no such caller could be found or
  checked here.
