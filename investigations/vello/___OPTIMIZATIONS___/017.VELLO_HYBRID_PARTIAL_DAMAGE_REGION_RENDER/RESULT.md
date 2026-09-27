# vello_hybrid: partial (damage-region) render + integer rect-clip fast path (PRs #1737, #1786)

`vello_hybrid`'s `Renderer::render` always redraws the whole target every call. Two open,
stacked PRs change that for the common "scrolling list / clipped viewport" UI-redraw shape:
#1737 (on top of #1783) adds a `RenderRegion` parameter so only a set of damage rectangles is
redrawn (scissored, with root-strip culling on the CPU side), and #1786 (on top of #1784) adds
detection of axis-aligned integer-coordinate clip rectangles so rect/texture-rect/glyph content
under such a clip is clamped analytically instead of falling off the GPU fast path onto the CPU
strip+mask pipeline. This candidate measures both, together, as they'd actually be used.

Toolchain: `cargo 1.98.0 (797e8a9bc 2026-08-05)`, `rustc 1.98.0 (88d9e12ae 2026-08-18)`.

## What was tried

No existing project benchmark reaches `vello_hybrid`'s GPU render path with a controllable
damage region or clip shape (`sparse_strips/vello_bench` only benches the CPU-only `vello_cpu`
renderer, no `wgpu` dependency at all), so a standalone example (`bench/bench_damage_*.rs`,
mirroring the project's own `examples/render_to_file.rs` device/texture setup) was written for
each side of the comparison and timed with `Instant`, 300 iterations, discarding the first 30 as
warmup, real `wgpu` device (falls back to software `llvmpipe`/`swiftshader` in this sandbox - see
"Why the result looks the way it does").

Scene: a 1920x1080 "list" of 400 rows (two `fill_rect` calls per row - a background bar and an
accent chip), built once and rendered repeatedly via `render()` on the same `Scene` (no
re-encoding per iteration, isolating the scheduler/execute/GPU-submit cost `RenderRegion`
actually changes). `partial` mode passes a single `RectU16` damage rect covering one row
(1920x28 px, ~2.6% of the target) with `ClearSettings::Rects` over the same rect - "one row
scrolled into view" from the PR's own example. A second scene (`clip`/`clip_rect`) fills 400
rows through an axis-aligned integer clip rect, once via a plain clip *path* (exercises the new
PR's auto-detection of an integer rect from an ordinary clip path) and once via the new explicit
`Scene::push_clip_rect` API.

Two independent measurement passes were taken: one directly in this session's own worktrees
(`taskset`-pinned single-core and unpinned multi-core, shared machine, load ~12/24 cores), and a
second, fully independent one via `just test` from a scratch clone (see justfile) - both agree on
direction and rough magnitude.

| Variant | multi-core p50 (worktree run) | single-core p50 (worktree run) | `just test` scratch-clone p50 |
|---|---|---|---|
| baseline: full render every frame | 4.08-4.30 ms (2 runs) | 37.06-38.29 ms (2 runs) | 6.07 ms |
| patched: full render, `RenderRegion::Full` (control) | 4.08-4.12 ms (2 runs) | 37.06-37.25 ms (2 runs) | 5.54 ms |
| patched: `RenderRegion::Rects`, 1 row / 1080 damaged | 0.56-0.57 ms (2 runs) | 1.49-1.53 ms (2 runs) | 1.07 ms |
| baseline: path-based clip (no rect fast path) | 6.86-6.92 ms (2 runs) | 47.15-47.45 ms (2 runs) | 9.52 ms |
| patched: same path clip (auto-detected as integer rect) | 2.65-2.70 ms (2 runs) | 23.22-23.35 ms (2 runs) | 3.48 ms |
| patched: explicit `push_clip_rect`, same scene | 2.70-2.73 ms (2 runs) | 23.18-23.20 ms (2 runs) | 3.55 ms |

`partial_renders()`/`culled_strips()` counters (patched build only) confirm the mechanism
engaged: `partial_renders=300` (one per iteration) and `culled_strips=231900` (773 strips culled
per iteration) for the `partial` mode.

## Result

Time only (no memory/allocation metric instrumented for this candidate).

- **Damage-region render** (`RenderRegion::Rects` over ~2.6% of the target vs. a full render of
  the same scene): multi-core p50 drops from ~4.1-4.3 ms to ~0.56-0.57 ms (**~7.2-7.7x**),
  single-core p50 from ~37-38 ms to ~1.49-1.53 ms (**~24-25x** - CPU-side scheduling/culling cost
  scales with damage area, so it benefits even more once GPU-parallel work is pinned out).
  Consistent across both the worktree runs and the independent `just test` scratch-clone run
  (6.07 ms -> 1.07 ms, ~5.7x - lower ratio on a busier/different machine state, same direction).
- **Full-frame render, control** (baseline vs. patched `RenderRegion::Full`): no regression -
  multi-core p50 ~4.08-4.30 ms both sides, single-core p50 ~37.06-38.29 ms both sides. The new
  API adds no measurable overhead when not used for a partial render.
- **Rect-clip fast path**: multi-core p50 drops from ~6.86-6.92 ms to ~2.65-2.73 ms
  (**~2.5-2.6x**), single-core p50 from ~47.15-47.45 ms to ~23.18-23.35 ms (**~2.0x**). Notably,
  the *plain path-based* clip already gets this win in the patched build (2.65-2.70 ms), same as
  the explicit `push_clip_rect` call (2.70-2.73 ms) - the PR's auto-detection ("integer rect
  detected from a pushed clip *path*, not just from `push_clip_rect`") is doing the actual work
  here; the explicit API is a convenience/documentation surface for callers who already know
  their clip is a rect, not a separate faster code path for this scene.

All deltas are far outside the run-to-run noise seen within each variant (each pair of same-
variant runs differs by <2% multi-core, <1% single-core), and the independent scratch-clone run
reproduces the same direction and order of magnitude.

## Does it change behavior/output?

- **Bit-exact?** N/A for the bench itself (no pixel hash was taken there), but the answer is
  **yes** per the project's own dedicated correctness suite, run as part of this candidate's
  test-suite check (see below): `partial_render.rs`'s `partial_render_damage_preserves_and_matches_full`
  and related tests assert byte-for-byte equality (0 differing bytes) between a `RenderRegion::Rects`
  render and a full render of the same scene, across disjoint and overlapping multi-rect cases;
  `clip_exact.rs` and `clip_rect_cpu.rs` assert the same for the rect-clip fast path (integer,
  fractional-under-#1784, nested-under-a-path-clip, rotated-fallback, and replay-across-filter-
  layer cases) - all pass (see below).
- Project test suite (`cargo test --release -p vello_hybrid -p vello_sparse_tests`):
  - Baseline (`896806de`, see "Why the result looks the way it does" for why this isn't the
    workspace's pinned `base_commit`): `vello_hybrid` unit tests **115 passed, 0 failed**; doc
    tests 1 passed; `vello_sparse_tests` **89 passed, 3531 failed, 1176 ignored** (4796 total).
  - Patched (`9f56470c`): `vello_hybrid` unit tests **120 passed, 0 failed** (+5 new tests, all
    passing); doc tests 1 passed; `vello_sparse_tests` **104 passed, 3579 failed, 1188 ignored**
    (4871 total, +75 vs. baseline - the new PNG-snapshot-backed tests the PRs add).
  - Every one of the 3531/3579 `vello_sparse_tests` failures, both before and after, is the same
    pre-existing environment issue, unrelated to this candidate: `git-lfs` is not installed in
    this sandbox, so the tracked reference PNG snapshots are unreadable LFS pointer files
    (`Unsupported(UnsupportedError { format: Unknown, kind: Format(Unknown) })` /
    `Invalid PNG signature`) - confirmed identical in kind to the same issue noted in
    candidate 007's `RESULT.md`.
  - The 15 tests specific to this feature (`partial_render::tests::*` x5, `clip_exact::tests::*`
    x7, `clip_rect_cpu::*` x3) do not depend on any PNG snapshot (they compare rendered pixmaps
    to each other in-process) and all **pass** in the patched build.
- **Is this a breaking change?** No observed API break for existing callers: `Renderer::render`
  gains two new trailing parameters (`clear: ClearSettings`, `region: RenderRegion`), which is a
  source-breaking signature change for any external caller of that specific method (not
  semver-breaking-in-spirit for a pre-1.0 crate, but every call site does need updating - this
  repo's own examples/tests in the diff update all of theirs). `Scene::push_clip_rect` and the
  `partial_renders()`/`culled_strips()` counters are pure additions.

## Why the result looks the way it does

`RenderRegion::Rects` lets the scheduler (`schedule`/`execute`) cull root-destined strips,
coverage runs, and rect/gap quads that fall entirely outside the damage rects before they reach
the GPU (`culled_strips=231900` above, i.e. 773/iteration for a scene with 800 rows' worth of
strip data outside a 1-row-tall damage rect) - the GPU pass itself is also scissored per rect, so
even the remaining, non-culled draw calls touch far fewer pixels. Both the CPU-side culling and
the GPU-side scissoring scale with damage area rather than scene size, which is exactly why the
single-core ratio (~24x, CPU-bound) is larger than the multi-core one (~7x, where the unclipped
full-render baseline already benefits from more available parallelism on this sandbox's
software rasterizer).

The rect-clip win has a different mechanism: before this patch, *any* active clip - even one
that's a trivial axis-aligned integer rectangle - demotes `fill_rect`/`draw_texture_rects`
content from the GPU analytic-edge quad path onto the CPU strip renderer plus a rasterized clip
mask intersection. `ClipContext` in #1786 detects the common case (the clip stack is a
pairwise-disjoint set of integer rects) and clamps content to it analytically instead, which is
why *both* the plain path-clip and the explicit `push_clip_rect` scene get the same ~2-2.6x win
here: the detection, not the new API surface, is what actually changes the code path taken.

This sandbox has no real GPU (`wgpu` falls back to a software adapter - `RUST_LOG=debug` was not
re-checked for this specific candidate, but the absolute millisecond values are broadly
consistent with candidate 007's own confirmed llvmpipe/swiftshader fallback measurement in this
same environment), so absolute numbers are not representative of real-hardware GPU timings, but
the *relative* deltas reflect real CPU-side scheduling/culling work and real GPU-side pixel/quad
volume reduction, both of which apply on real hardware too.

## Why `patch.diff`/`base_commit` differ from this workspace's usual shape

The four upstream commits that make up this feature (#1783 "Make clear behavior more
customizable", #1737 "Partial (damage-region) rendering via RenderRegion", #1784 "Make GPU rect
quads byte-identical to the CPU strip path", #1786 "Rectangle clips: integer fast path and
push_clip_rect") are stacked on a common ancestor, `896806de` ("feat: allow disabling
vello_hybrid depth buffers (#1808)"), which is **34 commits behind** this workspace's pinned
`base_commit` (`75168314`). `git apply --check patch.diff` against a checkout of `75168314`
fails (conflicts in `sparse_strips/vello_hybrid/{examples/*, src/render/webgl/*, src/render/wgpu/mod.rs,
src/scene.rs, src/draw.rs}` and `vello_sparse_tests/tests/renderer.rs`); against `896806de` it
applies cleanly (verified with `git apply --check`, both directly and via this candidate's own
`justfile`'s `show-patch` recipe). The 34 intervening commits are almost entirely WebGL-backend
work (data-texture bindings, non-blocking shader compilation, external-texture support) unrelated
to the wgpu-backend path this candidate measures, plus one CPU-side change
(`vello_common: Optimize pixmap unpremultiplication`, #1839) that doesn't touch the render/
schedule/clip code this candidate's scenes exercise. `patch.diff` is the real `git diff` between
`896806de` and the PR #1786 head commit (`9f56470c`) - not hand-constructed - and both the
worktree measurements above and the `justfile`'s own `just test` build directly from these two
real commits (`actual_base_commit`/`patched_commit`) rather than from `base_commit` + `apply`.

## What's not done / open questions

- Only one damage-region shape (a single thin top-row rect) and one clip shape (a single large
  centered rect) were measured. The PRs' own description also covers disjoint multi-rect damage
  and nested/rotated clip fallback - covered by their correctness tests above but not separately
  benchmarked here.
- `BumpAllocators`/strip-buffer peak sizes were not read back; the culled-strip count strongly
  suggests a proportional CPU-side memory-traffic reduction too, but this wasn't measured
  directly.
- The WebGL backend is explicitly out of scope for #1737 itself ("untouched for now") and was not
  exercised by this candidate either - only the wgpu backend was measured.
- Construction-time (clip-based) culling mentioned as a #1737 follow-up in the PR discussion was
  not investigated here.
- Both PRs are open/unmerged as of this measurement (`#1737`: `CONFLICTING` merge state against
  current `main`; `#1786`: stacked on top of it) - this candidate's numbers describe the feature
  as it exists on those branches today, not a state that can currently be applied to `main` as-is
  without the same rebase work a maintainer merging them would also need to do.
