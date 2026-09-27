# PATH_TILING_CPU_SHIM_ASSERT_AND_INDEX_WRITE

`vello_shaders/src/cpu/path_tiling.rs`'s `path_tiling_main` (the real CPU-shader fallback for
`path_tiling.wgsl`, used whenever `vello::RendererOptions::use_cpu` is `true`) computes `p0`/`p1`
via float arithmetic on external path geometry (including a `ROBUST_EPSILON` correction), then
`assert!`s each coordinate is within `[0, TILE_WIDTH]`/`[0, TILE_HEIGHT]`, then does an unchecked
`segments[(seg_start + seg_within_slice) as usize] = segment` write into a slice sized by an
upstream bump-allocator estimate with no bounds check of its own - a pattern not present in
`path_tiling.wgsl` itself (no `assert!`s there at all, and wgpu/naga's own storage-buffer bounds
checks make an equivalent WGSL OOB index a silent clamp, not a crash). This candidate checked
whether real, caller-supplied path geometry can trip either the asserts or the final write.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `path_tiling.rs` against `path_tiling.wgsl` line-by-line, and read the historical GitHub issues this exact assert produced (`gh search issues "path_tiling" --repo linebender/vello`) | Reading only | Found vello#378 and vello#379 (both closed, 2023): real CPU-shim assertion failures at this exact file/line from real scenes (a near-horizontal huge-span line; a NaN-producing stroke-to-fill bug). Both were fixed - #378 by adding the `ONE_MINUS_ULP`/`ROBUST_EPSILON` correction that is present, unmodified, in the current `path_tiling.rs`/`path_tiling.wgsl`; #379 by a separate kurbo stroke-expansion fix (kurbo#314, vello#396) that stopped NaN from ever reaching this code. Also found vello#775 (still **open** upstream): a *visual* tile-boundary-snapping robustness report, not a crash report, that the maintainer explicitly ties to "tile numerical robustness" work-in-progress. |
| Real end-to-end reproduction of #378's and #775's exact reported geometry, via `vello::Scene::fill` (public API) -> real `wgpu` device (SwiftShader/Vulkan software ICD) -> `vello::Renderer::render_to_texture` with `RendererOptions::use_cpu: true`, `catch_unwind` around the call | Release build, `WGPU_BACKEND=vulkan`, exact point coordinates from both issues | Neither panics (3/3 runs) - confirms the historical fix is still effective for both reported inputs in the current tree. |
| Real end-to-end reproduction, ~1200 randomized near-degenerate closed-quad paths (near-axis-aligned / near-diagonal short edges with `[1e-4, 1e-1]` jitter) at translation magnitudes 10 through 3e5 (mimicking the *shape* of #378/#775's reports and #1139's large-CAD-coordinate report), same `use_cpu:true` harness | Release build, xorshift-seeded, `WGPU_BACKEND=vulkan`, ~5 min | 0 panics, 0 `Err(..)` results across all ~1200 cases. |
| A systematic hand-constructed sweep (~320 cases) of near-grid-aligned "thin sliver straddling a tile boundary" shapes at 5 grid offsets x 8 epsilons x 4 magnitude scales, same `use_cpu:true` harness | Release build, `WGPU_BACKEND=vulkan` | 14 panics, **all** in a *sibling* file, `vello_shaders/src/cpu/path_count.rs:146` (`tile[(base + x_bump) as usize].backdrop += delta` - the identical "unchecked bump-allocator index" bug class, one pipeline stage earlier), 0 in `path_tiling.rs`. |
| Offline pure-math brute-force search (`repro/src/bin/search.rs`, no GPU/pipeline): `path_tiling_main`'s coarse-rasterization formula copied verbatim, checking every `seg_within_line` value in `0..count` for a resulting tile-relative `p0`/`p1` outside `[0,16]`, over 9 magnitudes (1 to 3e7) x up to 2,000,000 random (partly grid-snapped, partly axis-biased) `(p0,p1)` pairs each | Release build, ~2.6s total, ~18M trials | 0 violations found with the real `ROBUST_EPSILON` fix active. **Sanity check**: the identical harness, run with the fix's two lines commented out, finds violations within the first ~1M trials at every one of the first 4 magnitudes tested (19 found by mag=1000) - confirming the harness genuinely detects this failure mode and the negative result above reflects the fix working, not a broken checker. |
| Real end-to-end buffer-overflow attempts targeting the `segments`/`seg_counts` bump-allocated buffers (`vello_encoding::config`, both hardcoded to `1 << 21` = 2,097,152 entries) specifically, via `use_cpu:true` scenes designed to maximize segment count while minimizing tile-allocation cost (see `repro/src/bin/pipeline_checks.rs` parts 2a/2b) | Release build, `WGPU_BACKEND=vulkan`, 3/3 runs | Both constructions panic - but structurally *before* `path_tiling.rs` ever runs: 2a (many draws with near-full-canvas bboxes) overflows `tiles` in `tile_alloc.rs:52` after ~130 draws; 2b (one path, many full-diagonal lines, engineered specifically to decouple tile-allocation cost from segment count - see "Root cause") overflows `seg_counts` in `path_count.rs:150`, one pipeline stage before `path_tiling.rs`'s own `segments` write would need to. No construction found that reaches `path_tiling.rs`'s write with an out-of-range index. |

Five escalating approaches (issue-history reading, targeted real-pipeline reproduction of the
exact historical reports, broader real-pipeline randomized/systematic sweeps, an offline
brute-force search validated against a disabled-fix control, and a deliberately-engineered
buffer-overflow attempt) were used - beyond the loop's 3-approach soft cap, continued because each
approach kept turning up a live signal (real, reproducible panics), just never in `path_tiling.rs`
itself.

## Status

**Not reproducible.** Every construction that could plausibly trip either `path_tiling.rs`'s
`assert!`s or its final unchecked `segments[...]` write either rendered successfully, or panicked
in a *different*, earlier CPU-shim pipeline stage (`tile_alloc.rs`, `path_count.rs`) before
`path_tiling.rs` ran at all. See "Root cause" below for why this isn't coincidental. Every
approach above that produced a definite outcome was deterministic and reproduced identically
across at least 3 runs (see `output.txt`).

**Fully reproducible**: N/A (no bug found for this candidate's specific target - see Status).

## Class

N/A (not Confirmed).

## Root cause

Not a bug in the sense originally suspected, for two independent reasons:

1. **The `assert!` failure mode is fixed.** The two historical reports that hit these exact
   asserts (#378: a near-horizontal, huge-span, not-quite-grid-aligned line; #379: NaN from an
   unrelated stroke-expansion bug) are both closed. #378's fix - clamping `b` below 1 via
   `ONE_MINUS_ULP` and correcting `a` by `ROBUST_EPSILON` whenever `floor(a*(count-1)+b)` doesn't
   match the independently-computed tile-span (`count_x`) - is present unmodified in the current
   `path_tiling.rs`/`path_tiling.wgsl` and held up against both the original reports and an ~18M-
   trial offline search plus ~1500 real-pipeline randomized/systematic cases this session. #775
   (still open) is a different failure mode entirely - a *visual* winding-artifact report from
   tile-boundary snapping, not a crash - and did not additionally trip this file's asserts in this
   session's reproduction of its exact reported geometry.

2. **The unchecked-write overflow mode is structurally masked by earlier, identically-unchecked
   sibling writes.** `path_tiling.rs`'s loop runs exactly `bump.seg_counts` times, over entries
   that `path_count.rs` must have already written into the `seg_counts` buffer (also sized
   `1 << 21`, `vello_encoding/src/config.rs`) via its own equally unchecked
   `seg_counts[(seg_base + i - imin) as usize] = seg_count` - and the total demand on the
   `segments` buffer that `path_tiling.rs` writes into (`bump.segments`, accumulated per-tile in
   `coarse.rs`'s `write_path`) is bounded above by that same total (every segment `path_tiling.rs`
   would write was already counted into `bump.seg_counts` by `path_count.rs` first). Since
   `path_count.rs` runs strictly earlier in the pipeline (`vello/src/render.rs`'s dispatch order)
   and its own `seg_counts` buffer is the *same* size as `segments`, any scene large enough to
   overflow `segments` must have already overflowed `seg_counts` in `path_count.rs`, which panics
   first. Separately, `tile_alloc.rs` allocates tiles for a whole path's bbox *rectangle*
   (`(ux1-ux0)*(uy1-uy0)`, a product) while `path_count.rs`/`path_tiling.rs` only produce segments
   for tiles an actual line touches (`count_x+count_y`, a sum) - normally `tiles` fills up faster
   than `segments` for typical shapes, which is why this session's first overflow attempt (2a,
   many overlapping full-canvas draws) hit `tile_alloc.rs` first. Deliberately decoupling this (2b:
   one path whose bbox/tile cost is paid once, containing many independent full-span diagonal
   lines that each cost many segments) still hit `path_count.rs`'s `seg_counts` write instead,
   confirming the `seg_counts`/`segments` argument above rather than the `tiles`/`segments` one.

## Introduced by

Not applicable (no bug confirmed).

## The fix

Not applicable (no bug confirmed - see Status/Root cause).

- Project test suite: not run as part of this candidate (no fix to verify); see "What's not
  done" for the pre-existing, unrelated LFS/snapshot-test limitation noted in this workspace's own
  `README.md`.

## What's not done / open questions

- **`tile_alloc.rs:52`, `path_count.rs:146`/`:150`, `binning.rs:102`, and `coarse.rs:382`
  (`ptcl` buffer) all have the identical bug class** (an unchecked index write into a slice sized
  by a fixed-capacity, hand-picked bump-allocator constant from `vello_encoding::config`'s
  `BufferSizes::new`, with no bounds check of its own) and were all independently triggered as
  real panics via the real public API during this session's exploration - none of them were this
  candidate's target (`path_tiling.rs`), so none were root-caused, fixed, or written up here. Each
  is a legitimate, separate candidate for a future session; see `SUMMARY.md`'s "New suspects" (via
  `IDEAS.md`) for the exact repro shapes that hit each one (this candidate's `repro/` also
  demonstrates two of them concretely: `tile_alloc.rs:52` and `path_count.rs:150`).
- This is architecturally the same root cause the project's maintainers already track for
  `bin_data` specifically (see this workspace's own 027.GPU_BUFFER_BUMP_ALLOCATION_LIMITS_SILENT_STOP:
  vello#720/#1660, and aad292dd's `BumpAllocators.failed` mechanism) - on the **real GPU** path,
  every one of these same buffers has a `bump.failed` bit set and a substituted safe write location
  when its fixed-size capacity is exceeded, so none of them crash the GPU; the **CPU shim**
  (`vello_shaders/src/cpu/*.rs`) never implements that same `bump.failed` check anywhere (not in
  `path_tiling.rs`, nor in any of the sibling files above), so on CPU the identical overflow
  condition that's silently handled on GPU becomes a hard panic instead. A future candidate could
  usefully treat "does the CPU shim mirror every GPU shader's own `bump.failed`
  check" as one systematic question covering all of `tile_alloc.rs`/`path_count.rs`/
  `binning.rs`/`coarse.rs`/`path_tiling.rs` at once, rather than one candidate per file.
- vello#775 (open upstream) was read and its exact reported geometry was re-run through this
  session's `use_cpu:true` harness (see "What was tried"), confirming it does not crash
  `path_tiling.rs` - but this session did not check whether it produces the same *visual* artifact
  the issue reports (pixel-level comparison wasn't attempted, since that's a different question
  than this candidate's crash/UB-shaped one).
- The offline `search.rs` tool's random sampling, even biased toward near-grid-aligned/near-axis
  configurations, is not exhaustive - it is strong evidence the fixed asserts are robust in
  practice, not a proof for all possible `f32` bit patterns. A longer, dedicated fuzzing campaign
  (`fuzz-bug-repro`, run separately) targeting `path_tiling_main`'s formula directly (as a
  `#[no_mangle]`/`pub(crate)`-exposed function, since it's currently private) would be the natural
  next escalation if this is revisited.
