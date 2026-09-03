# STROKE_GPU_INNER_JOIN_MITER_HOLE

`vello`'s classic GPU compute-shader stroke expansion (`vello_shaders/shader/flatten.wgsl`'s
`draw_join`) renders an incorrect hole/notch at a sharp `Join::Miter` + `Cap::Butt` corner. This
looked suspicious because it's an already-filed, still-open issue
(https://github.com/linebender/vello/issues/1063) with a concrete repro and a maintainer comment
(raphlinus) explicitly diagnosing the mechanism ("processing of the inner join... both kurbo and
Vello GPU only guarantee weak correctness") and pointing at an unmerged draft fix, plus a later
comment (2025-10-26) noting the CPU renderer (`vello_cpu`) has since started rendering it
correctly while the classic GPU renderer had not - worth confirming that split is still true on
current `HEAD` and, if so, whether the GPU side has a tractable bounded fix.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Render the issue's exact repro through `vello_cpu`'s real public API (`RenderContext::stroke_path`/`render_with`) | debug build example, `sparse_strips/vello_cpu/examples/bughunt_1063.rs`, this candidate's own worktree | Did **not** reproduce - fully filled, 0 holes against an analytically-guaranteed-fill union of the two stroked segments' rectangles. Confirms the 2025-10-26 issue comment: the CPU/sparse-strips renderer (kurbo 0.13.1-based) already renders this input correctly on current `HEAD` |
| Render the exact same scene through the classic **GPU** `vello` crate's real public API (`Scene::stroke` -> `Renderer::render_to_texture`), driven by a real `wgpu` device (Mesa llvmpipe software rasterizer via the GL backend, `WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1`, since this sandbox has no real GPU) | release build, `examples/headless/src/bin/bughunt_1063.rs` scratch binary in this worktree | **Reproduced**: 210 background (black) pixels found strictly inside the region that must be filled regardless of join geometry (the union of the two stroked segments' own rectangles) - a visible diagonal notch cut into the corner, matching the issue's screenshot. Confirmed a real GPU-only regression relative to the CPU renderer |
| Repeatability (CONFIRMATION.md section 1) | Same binary, 4 runs total (1 initial + 3 repeats), release build, llvmpipe/GL | 4/4 reproduces, deterministic, exact same hole count (210) every run - no layout/seed dependency |
| Root-cause read of `vello_shaders/shader/flatten.wgsl`'s `draw_join` (`STYLE_FLAGS_JOIN_MITER` case) | Source reading | The miter case only computes a proper extended edge for the convex ("outer") side of a sharp join; the concave ("inner") side is connected with a single straight line directly through the join point without checking for self-intersection with the neighboring segment's own offset edges. Filled with the nonzero winding rule, that self-intersecting inner geometry can locally cancel to zero exactly where it overlaps the interior of the adjoining segment - punching the observed hole. This is the same "weak correctness" stroke model raphlinus described in the issue thread |
| Checked project history for a prior attempt at this exact fix | `git log --all -S"is_backside" -- '*flatten.wgsl'` | Found `24df185a`/`5061d038` ("Add inner joins", 2024-06-05, both by Raph Levien) implementing essentially this diagnosis behind an `#ifdef inner_join` shader permutation flag - confirmed via `git merge-base --is-ancestor` that **neither commit is an ancestor of current `HEAD`**; both live only on the unmerged `origin/paper-revisions` branch, corroborating raphlinus's own comment that this fix exists only as a draft |
| Bounded fix: adapted the same "double-draw + reverse-arc" self-intersection-cancelling technique from that draft commit directly (unconditionally, no `#ifdef`/shader-permutation plumbing) into current `flatten.wgsl`'s `STYLE_FLAGS_JOIN_MITER` case only (`Join::Bevel`/`Join::Round` left untouched) | Same worktree, `cargo build`, rebuilt shaders via `vello_shaders`' build-time codegen | Original repro: 210 holes -> 0 holes, 4/4 repeat runs after the fix, all still fixed |
| Varied the input within the same bug class (CONFIRMATION.md section 7): mirrored corner (opposite turn direction, exercises the `cr < 0` branch the original repro's `cr > 0` doesn't reach), a non-90-degree (~63 degree) sharp corner with the same short-segment-vs-width ratio | Same GPU/llvmpipe harness, `examples/headless/src/bin/bughunt_1063_variants.rs` scratch binary | Both variants genuinely reproduced pre-fix (190 and 31 holes respectively, confirmed against a pristine `git stash`-restored build) and were fully fixed post-fix (0 holes each) |
| Regression check: same exact original corner geometry with `Join::Bevel` and `Join::Round` instead of `Join::Miter` (paths the patch does not touch) | Same harness | Byte-for-byte identical hole counts before/after the patch (1495 and 208 respectively) - confirms the patch is scoped to the miter case only and doesn't change bevel/round rendering at all |
| Regression check: classic-vello's own `Ghostscript_Tiger` SVG example scene (300x300, `examples/headless`) | `cmp` on the two PNGs rendered before/after the patch | Byte-for-byte identical output |
| Project test suite | `cargo test --workspace --release`, `WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1`, with and without `VELLO_SKIP_LFS_SNAPSHOTS=all` | See "The fix" below for exact counts; identical failing-test-name sets before/after in both configurations |

## Status

**Confirmed**. Reproduces deterministically, 4/4 runs (1 initial + 3 repeats), release build,
against a real `wgpu` device (Mesa llvmpipe software rasterizer via the GL backend - this sandbox
has no real GPU hardware; any working `wgpu` backend, including a real Vulkan/Metal/DX12 GPU,
exercises the exact same compute-shader stroke-expansion code in `vello_shaders`).

**Fully reproducible**: **Yes**. `repro/src/main.rs` calls the classic `vello` crate's real,
public `Scene::stroke` and `Renderer::render_to_texture` entry points (the exact same code path a
real application embedding vello would use), with the byte-for-byte same stroke/path parameters as
issue #1063's own reported repro. This is a single-participant bug (a caller handing an ordinary
path+stroke style to the renderer's genuine public API) - there is no second participant to fake
here, so CONFIRMATION.md section 11's bar is satisfied trivially (the same "malformed input handed
to a parser's real public entry point" case it describes as always fine, just for a renderer
instead of a parser).

## Class

logic

## Root cause

`vello_shaders/shader/flatten.wgsl`'s `draw_join` function (specifically the
`STYLE_FLAGS_JOIN_MITER` case, though the same pattern also exists for `STYLE_FLAGS_JOIN_BEVEL`/
`STYLE_FLAGS_JOIN_ROUND`) computes the stroke outline at a segment join as two offset edges,
"front" and "back", one on each side of the path. At a sharp turn, one side is convex (the
"outer" side, correctly extended to a computed miter point / round arc / bevel edge) and the other
is concave (the "inner" side). Before this fix, the inner side was simply connected with a single
straight line directly through the join point (`p0`) with no check for whether that line
self-intersects the offset edges of the segments on either side of the join.

For a corner where the two segments are short relative to the stroke's half-width (exactly the
issue's repro: two ~10-unit segments at 50-unit width), that straight inner connector crosses
*into* the interior of the neighboring segment's own rectangle. The whole stroke outline is filled
as a single polygon under the nonzero winding rule; where the self-intersecting inner connector's
own loop overlaps a region that would otherwise have winding number 1 (correctly filled), the
loop's own contribution can cancel it back to 0, producing the observed hole/notch exactly where
the issue's screenshot shows one. This is precisely the "weak correctness" stroke model raphlinus
described in the issue thread: the renderer only guarantees a topologically-simple *outer* offset
curve, not a fully self-intersection-free stroke outline at concave inner joins.

Because `vello_cpu` (the sparse-strips CPU renderer) delegates stroke expansion to the external
`kurbo` crate instead of implementing its own, and current `kurbo` (0.13.1, this workspace's pin)
apparently no longer exhibits this exact defect for this input (per the issue's own 2025-10-26
comment, confirmed independently in this session - see "What was tried" above), the bug is now
GPU-renderer-specific: `vello_shaders/shader/flatten.wgsl` implements its own from-scratch GPU
stroke-expansion algorithm and never delegates to `kurbo` at all, so it did not inherit whatever
fix (or behavior change) made the CPU path correct.

## Introduced by

35fdf1e8 "[flatten]: Implement miter join" (2023-11-13, Arman Uguray) - the commit that first
implemented `STYLE_FLAGS_JOIN_MITER` support in the GPU stroke shader, including the
unconditional-straight-line inner-side connector that this candidate's fix replaces. Confirmed via
`git merge-base --is-ancestor 35fdf1e8 HEAD` (ancestor) that this is the actual code currently
shipping, and via `git log -S"STYLE_FLAGS_JOIN_MITER" --follow -- '*flatten.wgsl'` that no
intervening commit on the mainline branch changed this specific weak-correctness behavior. A
maintainer-authored draft fix for the same bug class (`24df185a`/`5061d038`, "Add inner joins",
2024-06-05) exists but was confirmed via the same ancestor check to be **not** merged into `HEAD`
- it lives only on the unmerged `origin/paper-revisions` branch.

## The fix

Added a new `draw_inner_join` helper in `vello_shaders/shader/flatten.wgsl` and used it, in place
of the previous single straight connector, for the concave ("inner") side of the
`STYLE_FLAGS_JOIN_MITER` case only (`Join::Bevel`/`Join::Round` are unmodified - see "What's not
done" below). Adapted from the never-merged `24df185a`/`5061d038` draft (see "Introduced by"), but
applied unconditionally rather than behind that draft's `#ifdef inner_join` shader-permutation
flag, since this workspace's current shader-compilation pipeline (`vello_shaders/src/compile/mod.rs`)
has changed substantially since 2024 and porting the permutation plumbing itself was out of scope
for a bounded fix.

The technique: instead of one straight line from `inner0` to `inner1` (which can self-intersect the
neighboring segments' own offset edges and get its coverage cancelled by the nonzero winding rule),
draw the same two straight edges (`inner0`->`p0` and `p0`->`inner1`) **twice** - each edge's own
winding contribution cancels against its exact duplicate, so the straight connector itself
contributes zero net coverage anywhere - and then draw a `flatten_arc` from `inner1` to `inner0`
wound in the *opposite* direction (negative angle) from the corresponding outer-side arc/miter
angle. That reverse-wound arc exactly cancels whatever spurious coverage the original (now-doubled,
self-cancelling) straight connector would otherwise have produced at the actual self-intersection,
without ever having to compute where that self-intersection point is.

Verified against more than the one triggering input (CONFIRMATION.md section 7): the exact issue
#1063 repro (`cr > 0` branch), a mirrored corner (`cr < 0` branch), and a non-90-degree (~63
degree) sharp corner with the same short-segment-vs-width ratio - all three reproduced before the
fix and were fully fixed after. `Join::Bevel` and `Join::Round` on the exact original corner
geometry (paths the patch does not touch) produced byte-for-byte identical output before/after,
and the classic-vello `Ghostscript_Tiger` SVG example scene rendered byte-for-byte identically
before/after.

- Project test suite (`cargo test --workspace --release`, `WGPU_BACKEND=gl LIBGL_ALWAYS_SOFTWARE=1`):
  - Without `VELLO_SKIP_LFS_SNAPSHOTS`: **562 passed / 3571 failed, both before and after**,
    identical set of 3571 failing test names (diffed byte-for-byte). Every one of these failures
    is this sandbox's own missing `git-lfs` install causing snapshot-comparison tests to load
    unresolved LFS pointer-text files instead of real reference PNGs (the panic messages name this
    explicitly: `"If this file is an LFS file, install git lfs..."`) - a pre-existing environment
    limitation, unrelated to this patch.
  - With `VELLO_SKIP_LFS_SNAPSHOTS=all` (the `vello_tests` crate's own documented escape hatch for
    exactly this situation): **all test targets pass cleanly, both before and after**, except
    `vello_sparse_tests` (which has no equivalent skip flag) still showing the same 89 passed / 3538
    failed, identical failing-test-name set, before and after. `vello_sparse_tests` doesn't even
    depend on `vello_shaders` (confirmed via `grep` on its `Cargo.toml`), so it cannot be affected by
    this patch regardless.

## What's not done / open questions

- The fix is scoped to `Join::Miter` only. `Join::Bevel` and `Join::Round` have the exact same
  "single straight connector on the inner side" pattern in `draw_join` and likely share the same
  bug class (self-intersection-driven cancellation at a sufficiently sharp, short-segment inner
  join) - this wasn't confirmed with a concrete repro this session, only left unmodified for
  regression-safety. A maintainer or a future candidate should check whether `Join::Bevel`/
  `Join::Round` can independently reproduce the same hole pattern.
- The fix is applied unconditionally rather than behind the original draft's `#ifdef inner_join`
  shader-permutation flag; porting that permutation mechanism to the current
  `vello_shaders/src/compile/mod.rs` pipeline (so the "always draw inner joins" behavior could be
  toggled/benchmarked independently) was out of scope for a bounded fix this session.
- Per raphlinus's own comment on the issue, the *proper* long-term fix is "strong correctness" per
  Nehab 2020 (computing exact inner-join trims/optimizing away unnecessary inner geometry), of
  which this patch's "always draw + cancel via reverse arc" is an intentionally simpler
  intermediate step (matching what raphlinus called "one possible intermediate state" in the
  issue thread) - it always emits extra geometry (4 lines + an arc) at every non-smooth join
  regardless of whether self-intersection would actually occur, which is correct but not optimal
  for render performance on paths with many joins.
- `vello_hybrid` (the WebGL/WebGPU-oriented sparse-strips hybrid renderer) wasn't checked
  independently; it doesn't depend on `vello_shaders` either, so this specific patch doesn't touch
  it, but whether it has its own, separate instance of the same "weak correctness" inner-join
  pattern in its own shaders is unexamined this session.
- This session's GPU testing used Mesa llvmpipe (software rasterizer) via wgpu's GL backend, since
  this sandbox has no real GPU; the bug is in shared WGSL compute-shader source compiled by
  `vello_shaders` for every wgpu backend, so it is expected to reproduce identically on a real
  GPU/Vulkan/Metal/DX12 backend, but this wasn't independently confirmed on real GPU hardware this
  session.
