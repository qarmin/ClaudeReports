# 06 - Reuse femtovg Paths instead of allocating one per primitive (plan, before the change)

## Baseline (new `rects` scene, added for this: a grid of rounded, bordered rectangles, all visible)

FemtoVG under Xvfb/llvmpipe, 3 000 rectangles, 8 frames, every frame really re-rendered
(the grid offset follows the benchmark's scroll property):

```
$ just frames-xvfb  # --mode rects --count 3000 --frames 8
later frames   44.25 ms average, 22.6 fps, 51 938 allocations per frame
```

Allocation sites (`--alloc-profile 3`, ~35 343 samples scaled, 3 000 rectangles => ~17
allocations per rectangle per frame):

| share | site | per rect |
| --- | --- | --- |
| 52.3% | `register_as_dependency_to_current_binding` (geometry/brush property reads) | ~6 |
| 6.4% | `Vec<femtovg::geometry::Position>` reserve | 1 |
| 6.4% | `Vec<femtovg::path::PackedVerb>` reserve | 1 |
| 6.4% | `Vec<femtovg::path::cache::Contour>` grow | 1 |
| 6.3% | `Vec<femtovg::path::cache::Point>` grow | 1 |
| 6.3% | `Vec<femtovg::renderer::Vertex>` reserve | 1 |
| 6.3% | `Vec<femtovg::renderer::Drawable>` reserve_exact | 1 |
| 3.2+3.2+3.1% | more `Vertex` vecs + `fill_path_internal` | ~3 |

The dependency nodes are the known arena issue (#12255). Everything else, ~9 allocations per
rectangle, is femtovg building fresh buffers for every primitive of every frame.

## What this change does

`i-slint-renderer-femtovg`'s `rect_with_radius_to_path` / `rect_to_path`
(`internal/renderers/femtovg/itemrenderer.rs:104-141`, 11 call sites) create a brand new
`femtovg::Path` per rectangle per frame, i.e. two `Vec` allocations each (`verbs`, `coords`),
plus the tessellation cache (`Point`, `Contour`) that `fill_path` builds inside it.

Plan: keep a pool of scratch `Path`s in the renderer and hand them out through an RAII guard,
so the `Vec` capacity survives across primitives and frames. femtovg has no `Path::clear()`,
so add one (it is our checkout in `../../../femtovg`).

- Step 1: `Path::clear()` in femtovg - clear `verbs`/`coords`, reset `last_pos`, drop the
  tessellation cache. Keeps capacity.
- Step 2: a thread-local pool + `ScratchPath` guard in the Slint renderer; `rect_to_path` and
  `rect_with_radius_to_path` return the guard. Expected: -2 allocations per rectangle.
- Step 3 (only if step 2 measures well): keep the `PathCache` buffers too (`Point`/`Contour`)
  by clearing them instead of dropping the cache. Expected: -2 more.

## Expected size of the win

At 34 ns per malloc+free pair (`just allocbench`), 4 of 17 allocations per rectangle is
~136 ns per rectangle per frame: 0.4 ms per frame at 3 000 rectangles. That is ~1% of the
44 ms llvmpipe frame here, but llvmpipe's CPU rasterization is what dominates that 44 ms -
on a real GPU the same frame is a few ms, where 0.4 ms is ~10-20%. Allocation *counts* are
GPU-independent, so this plan reports counts as the primary metric and time as secondary,
and the user should re-check time on real hardware.

## Verification

- `--mode rects` allocations/frame and time, 1 000 and 3 000 rectangles.
- `--mode list`, `--mode scroll`, `--mode text` unchanged (no visual regressions, same counts
  minus the path allocations).
- The repo's own femtovg tests: `cargo test -p i-slint-renderer-femtovg`, plus
  `cargo test -p femtovg` in the checkout for `Path::clear`.
- Visual: rectangles, borders and rounded corners must look identical (the scratch path must
  be cleared, or corners of the previous rectangle would leak into the next one).
