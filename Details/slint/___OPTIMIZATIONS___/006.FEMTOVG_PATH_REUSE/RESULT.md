# 06 - Reuse femtovg Paths instead of allocating one per primitive

Ported from `perf_test/___OPTIMIZATIONS___/06_femtovg_path_reuse/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

This is a "vendoring a suspect dependency" case (see `~/.claude/skills/perf-optimize/SKILL.md`'s
"Vendoring a suspect dependency" section): one of the two patches below touches femtovg's own
source, not Slint's. Together with candidate `011.FEMTOVG_BUFFER_REUSE`, which makes this one worth
6x more (-13.8 allocations per rectangle instead of -2). The numbers below are what this patch
alone was worth; see `011.FEMTOVG_BUFFER_REUSE/RESULT.md` for the combined result and for the
`[patch.crates-io] femtovg` entries every workspace needed.

`patch.diff` in this folder **was**, in the original session, the full renderer diff against git
HEAD, so it also contained candidates `009.GRADIENT_STOPS_VEC` and `010.GRADIENT_BBOX_TESSELLATION`
bundled in. The 2026-08-23 rebase (see that section below) replaced it with a standalone diff
scoped to only this candidate - see that section for why.
Plan and baseline as written before the change: `PLAN.md`.

## What was tried

Two patches, both included here:

- `patch_femtovg.diff` - adds `Path::clear()` to the femtovg checkout: clears `verbs`,
  `coords` and the tessellation cache but keeps the `Vec` capacity. **Applied** to the femtovg
  checkout (additive API, `cargo test --lib` in femtovg: 61 passed).
- `patch.diff` - `i-slint-renderer-femtovg`: a thread-local pool of scratch paths
  behind an RAII `ScratchPath` guard (Deref to `femtovg::Path`, returned cleared on drop), used
  by `rect_to_path`, `rect_with_radius_to_path` and `clip_path_for_rect_alike_item`. The pool
  is bounded to 8 paths so nested clips cannot grow it without limit. **Reverted** from the
  tree, see below.

## Result

Measured (FemtoVG under Xvfb/llvmpipe, `--frames 8`):

| scene | allocations/frame before | after | delta |
| --- | --- | --- | --- |
| `rects` 3 000 rounded+bordered rectangles | 51 938 | **47 422** | -4 516 (-8.7%) |
| `rects` 1 000 | 39 054 | **34 978** | -4 076 (-10.4%) |
| `list` 100 000 records (ListView, ~25 rows visible) | 3 898 | **3 637** | -261 (-6.7%) |

The saving is exactly 2 allocations per drawn rectangle (verbs + coords), which the site
profile confirms: `RawVec<femtovg::path::PackedVerb>` and `RawVec<femtovg::geometry::Position>`
(6.4% each before) are **gone** from the ranking afterwards.

Frame time is unchanged within noise here (44.3 vs 46.3 ms at 3 000 rects, 45.0 vs 45.3 in a
repeat), because llvmpipe's CPU rasterization dominates that frame: 4 516 allocations x 34 ns
(`just allocbench`) is ~0.15 ms of a 45 ms frame. On a GPU, where the same frame is a few
milliseconds, the same 0.15 ms is a few percent. **Allocation counts are hardware
independent - the time saving has to be re-measured on real hardware.**

```sh
just frames-xvfb-mode rects 3000 8      # or: perf_test --mode rects --count 3000 --frames 8
perf_test --mode rects --count 3000 --frames 6 --alloc-profile 3   # site ranking
```

## Does it change behavior/output?

- **Bit-exact?** Pixel comparison of the last frame, before vs after (`--snapshot`, added to
  perf_test for this):
  - `rects` scene: **byte-identical** PPM.
  - `list` and `text` scenes: differ by 132 and 1 158 bytes of 3 302 400 - but two runs of the
    *same* binary differ by 300 and 1 395 bytes, so the scenes are nondeterministic (text
    cursor blink / llvmpipe glyph timing) and the patch is well inside that noise.
- **Is this a breaking change?** No - `Path::clear()` is additive femtovg API; the renderer-side
  pool is an internal implementation detail with no observable behavior change.
- `cargo test --lib` in the femtovg checkout: 61 passed. `cargo test -p i-slint-renderer-femtovg`
  (with the root workspace temporarily patched to the local femtovg): compiles and runs (the
  crate has no unit tests of its own).

## Why the result looks the way it does

Each rectangle drawn by the FemtoVG renderer previously allocated a fresh `femtovg::Path`
(`verbs` + `coords` vectors) every frame. `Path::clear()` plus a thread-local scratch pool let
those two vectors' capacity survive across frames, removing exactly the 2 allocations per
rectangle the profile predicted.

## What's not done / open questions

`patch.diff` needs `femtovg::Path::clear()`, which exists only in the local
femtovg checkout, so it is **not applied** to the main tree as shipped. The repository's
`examples/`, `demos/` and `tests/` are separate workspaces that build `i-slint-renderer-femtovg`
against **crates.io** femtovg, so applying the renderer patch breaks them until `Path::clear()`
is released (or every workspace gets a `[patch.crates-io] femtovg`). `Path::clear()` itself is
additive and stays in the checkout - it is the piece to upstream first.

To try it: apply `patch.diff` and add to the root `Cargo.toml`
`[patch.crates-io] femtovg = { path = "femtovg" }` (perf_test already has it).

**Next step, not attempted:** the tessellation cache (another ~2 allocations per primitive). The
site ranking after this patch:

```
  ~   18474 ( 59.9%)  register_as_dependency_to_current_binding      # dep nodes, #12255
  ~    2247 (  7.3%)  RawVec<femtovg::path::cache::Point>::grow_one
  ~    2244 (  7.3%)  RawVec<femtovg::path::cache::Contour>::grow_one
  ~    2235 (  7.3%)  RawVec<femtovg::renderer::Drawable>::reserve_exact
  ~    2226 (  7.2%)  RawVec<femtovg::renderer::Vertex>::reserve
  ~    1122 (  3.6%)  Canvas::fill_path_internal
  ~    1110 (  3.6%)  Vec<Vertex> from_iter (PathCache::expand_fill)
  ~    1110 (  3.6%)  RawVec<femtovg::renderer::Vertex>::grow_one
```

`Point`/`Contour` are the `PathCache` that `fill_path` builds inside the path; `Path::clear()`
currently drops it (`cache = None`). Keeping those two vectors (clearing them instead of
dropping the cache, with the cache key invalidated so stale tessellation can never be served)
would remove ~2 more allocations per primitive - this is exactly what candidate
`011.FEMTOVG_BUFFER_REUSE` does. The `Vertex`/`Drawable` vectors above are femtovg's per-draw-call
render buffers - another ~3 per primitive, and the same pooling idea applies inside femtovg's
renderer.

## Reproducibility check after upstream update (2026-08-23)

`patch_femtovg.diff` (femtovg's own repo) still applies cleanly against femtovg HEAD `df54a3c`. `patch.diff` (the slint-side `i-slint-renderer-femtovg` change) no longer applies cleanly against slint HEAD `cdb6c43f7` - a single-hunk failure near its first context line (the surrounding file grew since two nearby upstream commits: `scalable_render_size` for SVG rasterization and box-shadow blur padding/sigma from core). Confirmed the underlying issue is unchanged: `femtovg::Path::new()` is still allocated at ~11 call sites in `itemrenderer.rs` (grep count on the current file), not pooled. **Still relevant, needs a rebase.** Not rebuilt/re-benched this round.

## Rebase onto master `cdb6c43f7` (2026-08-23, same session)

`patch.diff` and `justfile`'s `slint_base_commit`/`femtovg_base_commit` are now current
(`cdb6c43f7` / `df54a3c`, the two commits identified as still-relevant by the check above).

**Scope narrowed on rebase.** The old `patch.diff` in this folder was, as its own header already
said, "the full renderer diff against git HEAD" from the original session - it bundled this
candidate's `ScratchPath` pooling together with candidates `009.GRADIENT_STOPS_VEC` and
`010.GRADIENT_BBOX_TESSELLATION`'s `brush_to_paint`/gradient-stops changes, unlike 009's and 010's
own folders, which each carry their own standalone diff. That bundling turned out to matter for
the rebase: upstream has since replaced `brush_to_paint(&self, brush: Brush, path: &femtovg::Path)`
with `brush_to_paint(&self, brush: Brush, size: PhysicalSize)` behind a new
`i_slint_core::graphics::ResolvedBrush`/`to_femtovg_stops` abstraction (see the open backlog entry
this same re-verification pass logged for 009/010 - "Brush::stops() is no longer called directly
at the renderer site" - a genuinely new investigation, not a mechanical rebase). Carrying forward
the old bundled diff's `brush_to_paint`/`brush_to_paint_with_bounds`/`gradient_stops` hunks would
have meant re-deciding 009/010's design against that new abstraction as a side effect of rebasing
006, which is out of this task's scope. **This rebase's `patch.diff` is therefore standalone and
scoped to only what this candidate's own description covers**: the thread-local `PATH_POOL`,
the `ScratchPath` RAII guard, and changing `rect_with_radius_to_path`/`rect_to_path` to return
`ScratchPath` instead of allocating a fresh `femtovg::Path`. `clip_path_for_rect_alike_item`, the
third function the original description named, no longer exists at all - clipping was refactored
upstream (`visit_clip` now either calls `combine_clip`'s `intersect_scissor`, no path involved, for
an axis-aligned clip, or builds a path directly via `rect_with_radius_to_path` for the rounded/
layered case) - so pooling `rect_with_radius_to_path` already covers its former role, with no
separate function to change.

**Coverage is now wider than the original description**, as a side effect of upstream's own
refactors since this candidate was written: `rect_with_radius_to_path` is called from 4 sites in
current `itemrenderer.rs` (`draw_border_rectangle`'s background and border paths, the box-shadow
path, and `visit_clip`'s rounded-layer path), plus `rect_to_path` in `draw_rectangle` - all 5
benefit from the pool automatically because they all go through these two functions; none of them
needed their own call-site change.

Verified after the rebase (both repos reverted to clean afterwards):
- `patch_femtovg.diff` applied to the femtovg checkout at `df54a3c`: `cargo test --lib` **76
  passed, 0 failed** (was 61 at the time of the original measurement; the femtovg checkout has
  grown its own test suite since, unrelated to this patch). Needed a temporary `[workspace]` table
  in the checkout's own `Cargo.toml` to run standalone (it now sits nested inside the slint
  checkout's directory tree, which otherwise makes Cargo treat it as an unlisted workspace
  member) - reverted after the test run, not part of the shipped patch.
- `patch.diff` applied to `internal/renderers/femtovg/itemrenderer.rs` at `cdb6c43f7`:
  `cargo build --bin perf_test --profile rdebug --features software` (which also builds the
  default `femtovg` feature) compiles cleanly with both patches applied together.
- Re-ran the allocation-profile comparison this candidate's original measurement used
  (`perf_test --renderer femtovg --mode rects --count 3000 --frames 8 --alloc-profile 3`,
  `LIBGL_ALWAYS_SOFTWARE=1` under `xvfb-run`, 1280x900 screen), baseline vs. rebased patch, same
  `cdb6c43f7` source otherwise:

  | | baseline | patched | delta |
  | --- | --- | --- | --- |
  | later frames, allocations (median-ish, 1 run) | 62 162..80 163 | 57 700..75 701 | -4 462 (-7.2% at the low end) |
  | allocation profile sampled total | ~62 295 | ~57 834 | -4 461 (-7.2%) |
  | later frames, median frame time | 54.5-55.7 ms | 54.3-55.1 ms | unchanged within noise (llvmpipe-dominated, as the original result noted) |

  The site ranking confirms the same mechanism as the original measurement: `RawVec<femtovg::
  path::PackedVerb>::reserve` and `RawVec<femtovg::geometry::Position>::reserve` (the verbs and
  coords vectors) are **gone** from the patched profile's top sites, exactly as before. The percentage
  is slightly smaller than the original -8.7% because the current `rects` scene profile has a larger
  denominator (upstream added `visit_clip`/`visit_opacity`/`clips_children` allocation sites not
  present in the original session's profile) - not because less was saved in absolute terms (-4 461
  here vs. -4 516 originally, the same order of magnitude for the same scene and count).
- `--snapshot` pixel comparison of the last frame, baseline vs. patched, same conditions: **byte-
  identical** PPM output, matching the original candidate's bit-exact `rects`-scene result.
- Working tree reverted in both repos (`git checkout -- internal/renderers/femtovg/itemrenderer.rs`
  in the slint checkout, `git checkout -- src/path.rs` in the femtovg checkout) after measuring.
  `patch.diff` here is against `slint_base_commit` `cdb6c43f7` (updated in `justfile`, along with
  `femtovg_base_commit`), verified to `git apply --check` cleanly against a fresh worktree at that
  commit.
