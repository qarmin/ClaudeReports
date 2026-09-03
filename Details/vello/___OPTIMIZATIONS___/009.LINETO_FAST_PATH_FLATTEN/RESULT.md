# LINETO fast path in the `flatten` CPU shader mirror

Every `PATH_TAG_LINETO` segment is degree-raised to a cubic and pushed through the full
Euler-spiral flatten machinery (`CubicParams::from_points_derivs` - 2x `atan2`, 2x `sin`/`cos`,
several `sqrt`/`hypot` - plus `EulerParams::from_angles`, the ESPC-integral branch, and
`scale_multiplier`'s own `sqrt`) even though the degree-raised control points are always exactly
collinear, so the fit trivially converges to a single line equal to the chord. For strokes this
runs twice per segment (once per offset curve). Source: linebender/vello#1633. Tested against
`vello_shaders/src/cpu/flatten.rs`, the CPU mirror of `vello_shaders/shader/flatten.wgsl` (no
GPU/wgpu adapter was available in this environment - see "What's not done" below).

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

A new example harness, `vello_shaders/examples/bench_flatten.rs` (added by `patch.diff`, no
suitable existing bench reached this code path - see README note below), drives the real
`pathtag_reduce` -> `pathtag_scan` -> `flatten` CPU shader chain directly (no GPU needed) over a
deterministic GIS/vector-map-style scene: 2000 paths, 48 dead-straight zig-zag `LineTo` segments
each, both as fills and as 3px-wide strokes (the latter exercising the fast path twice per
segment, once per offset curve, plus miter joins on the un-fast-pathed corners).

Fast path added: `CubicPoints` now carries `is_line: bool` (true for a real `LINETO`, including
the synthetic closing line `read_path_segment` already builds for a stroked cap marker's closed
subpath). `flatten_euler` checks it right after the existing zero-length-line early return, and
when true, emits the chord (`t_start`/`t_end`, or swapped for a negative offset, matching exactly
what the general loop already reduces to on its first iteration when `n == 1`) directly, skipping
`CubicParams`/`EulerParams`/the ESPC-integral loop entirely.

All timings: `bench_flatten <mode> 2000 48 30`, `flock`-serialized against sibling candidates
running in the same batch, min/p50 read off 30 iterations with the first 3 discarded as warmup.

| Variant | mode | single-core p50 (`taskset -c 0`) | unpinned p50 | notes |
|---|---|---|---|---|
| baseline (run 1) | fill | 10.845 ms | - | |
| baseline (run 2) | fill | 10.721 ms | 10.705 ms | |
| patched (run 1) | fill | 6.417 ms | - | |
| patched (run 2) | fill | 6.431 ms | 6.480 ms | |
| baseline (run 1) | stroke | 53.378 ms* | - | *noisy, sibling batch contention; min 44.358 ms |
| baseline (run 2) | stroke | 42.601 ms | 43.251 ms | |
| patched (run 1) | stroke | 34.166 ms | - | |
| patched (run 2) | stroke | 34.298 ms | 34.560 ms | |

The `flatten` CPU shader (and the `pathtag_reduce`/`pathtag_scan` steps that feed it) are plain
single-threaded Rust with no internal parallelism, so the "unpinned/multi-core" column is included
per the loop's requirement but isn't expected to differ from the pinned single-core number beyond
ordinary system noise - and it doesn't, in either build.

## Result

**Fill** (pure `LineTo` polygons, offset==0 so only one `flatten_euler` call per segment): p50
10.7-10.8 ms -> 6.42-6.48 ms, about **40% less time** (~1.66x).

**Stroke** (3px-wide, so two offset-curve `flatten_euler` calls per segment plus a bevel/miter
join per vertex, none of which hit the fast path): p50 42.6-43.3 ms (excluding one visibly noisy
53 ms baseline run caused by sibling-candidate contention right before this run's own `flock`
acquisition; that run's own **min** of 44.4 ms is consistent with the other three) -> 34.2-34.6 ms,
about **20% less time** (~1.25x). Smaller than the fill case because a larger share of per-segment
stroke cost (join/cap geometry, tangent/normal computation, two `Transform::apply` calls) sits
outside `flatten_euler` and is unaffected by this patch.

Both deltas are far larger than the run-to-run spread seen within each variant (well under 1% p50
spread across the two repeat runs of each build), so the direction and rough magnitude are not
noise.

Only time was measured; this patch touches no allocation pattern (no heap allocation in either
version of `flatten_euler`), so no memory dimension applies.

## Does it change behavior/output?

- **Bit-exact?** Yes, confirmed on this bench's two scenes (fill and stroke): the FNV-1a64 hash
  over the resulting `LineSoup` array (truncated to `bump.lines`) XORed with a hash over
  `PathBbox`, computed by `bench_flatten`, is identical between the baseline and patched builds
  (`7c501deaa812a4f8` for fill, `355d8a80a594f3a9` for stroke), across all 30 iterations of each
  run.
- Reasoning for why this holds generally, not just on this bench's scene: for a real `LINETO`,
  the degree-raised cubic's control points are exactly collinear by construction
  (`p1 = mix(p0,p3,1/3)`, `p2 = mix(p3,p0,1/3)`), so `CubicParams::from_points_derivs` always
  computes `th0 == th1 == 0` up to floating-point rounding, `err` at or extremely close to `0.0`
  (well under the `0.25` tolerance on the very first, unsubdivided iteration), and `k0 == k1 == 0`
  up to the same rounding - which forces `n == 1` after `.ceil().clamp(1.0, 100.0)`. With `n == 1`
  the existing loop's own last-iteration special case (`i == n-1 && t1 == 1.0`) already sets
  `lp1 = t_end` without ever calling `es.eval_with_offset`, so the *only* output the general path
  can produce for a real `LINETO` is the same chord line this patch emits directly - the two paths
  are mathematically the same function, just computed with the expensive machinery elided. The one
  theoretical (not observed) gap: astronomically large offset/chord-length ratios could in
  principle amplify floating-point noise in `k0`/`k1` enough to round `n` up above 1 in the general
  path; no realistic stroke-width/geometry ratio triggers this, and the bench scenes above did not.
- **Is this a breaking change?** No - internal CPU shader implementation detail, no public API or
  output format change.
- Project test suite (see "What's not done" for why the full workspace suite isn't a clean
  baseline in this environment): scoped to the packages this patch can plausibly affect
  (`vello`, `vello_encoding`, `vello_shaders`, `vello_tests`, run with `VELLO_CI_GPU_SUPPORT=no`
  since no GPU adapter is available here) - **31 passed, 0 failed, 67 ignored (GPU tests), both
  before and after**, identical. Full-workspace `cargo test --workspace --release` (also
  `VELLO_CI_GPU_SUPPORT=no`) reports **89 passed, 3538 failed, 1230 ignored, both before and
  after**, byte-for-byte identical failure count - every one of those 3538 failures is
  `vello_sparse_tests` (a completely separate rendering engine, `sparse_strips/vello_cpu`, that
  never calls into `vello_shaders`) failing with `Unsupported(UnsupportedError { format: Unknown
  ... })` while loading its reference PNG snapshots, because `git-lfs` isn't installed in this
  environment (the repo's own `post-checkout` hook already warns about this on every checkout) so
  those snapshots are unresolved LFS pointer files, not images - confirmed pre-existing and
  unrelated to this patch, not a new regression.

## Why the result looks the way it does

The Euler-spiral fit machinery being skipped (atan2/sin/cos/sqrt/hypot calls, plus the
`EulerParams::from_angles` polynomial and the ESPC-integral branch) is genuinely expensive
relative to a single line-segment output, and a straight `LineTo` never needed any of it - the fit
was always converging to the trivial answer, just by a very roundabout, transcendental-function-
heavy path. The stroke case's smaller relative win matches expectation: `flatten_euler` is only
part of per-segment stroke cost (join computation, tangent/normal setup, and two
`Transform::apply` calls per emitted line happen regardless).

## What's not done / open questions

- **`vello_shaders/shader/flatten.wgsl` (the actual GPU shader) was not patched.** No GPU/wgpu
  adapter was available in this sandboxed environment (no discrete GPU, no software Vulkan ICD
  such as lavapipe installed), so a WGSL change could not be measured or correctness-checked here
  (`vello_tests/tests/compare_gpu_cpu.rs` and friends need a real adapter and were not run - see
  the identical `ignored` counts above under `VELLO_CI_GPU_SUPPORT=no`). The CPU mirror in
  `vello_shaders/src/cpu/flatten.rs` is the one measured and correctness-checked in this
  candidate; porting the same `is_line`/fast-path structure to the WGSL source is mechanical but
  unverified.
- The bench only covers dead-straight zig-zag polylines at one width (3px) and one segment count
  (48/path); it does not sweep stroke width, join style, or a mix of `LineTo`/`QuadTo`/`CubicTo`
  segments in the same path (the fast path only ever triggers per-segment, so mixed paths should
  see a proportional win, but this wasn't separately measured).
- No memory/allocation dimension was measured (see "Result" - not applicable, no allocation in
  either version of the touched function).
