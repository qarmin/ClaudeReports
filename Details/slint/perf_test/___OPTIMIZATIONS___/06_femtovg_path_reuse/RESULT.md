# 06 - Reuse femtovg Paths instead of allocating one per primitive

**Status: win, APPLIED to the tree** together with candidate 11, which makes it worth 6x more
(-13.8 allocations per rectangle instead of -2). The numbers below are what this patch alone was
worth; see `../11_femtovg_buffer_reuse/RESULT.md` for the combined result and for the
`[patch.crates-io] femtovg` entries every workspace needed.

`slint_scratch_path.diff` in this folder is now the full renderer diff against git HEAD, so it
also contains candidates 09 and 10.
Plan and baseline as written before the change: `PLAN.md`.

## The change

Two patches, both included here:

- `femtovg_path_clear.diff` - adds `Path::clear()` to the femtovg checkout: clears `verbs`,
  `coords` and the tessellation cache but keeps the `Vec` capacity. **Applied** to
  `../../../femtovg` (additive API, `cargo test --lib` in femtovg: 61 passed).
- `slint_scratch_path.diff` - `i-slint-renderer-femtovg`: a thread-local pool of scratch paths
  behind an RAII `ScratchPath` guard (Deref to `femtovg::Path`, returned cleared on drop), used
  by `rect_to_path`, `rect_with_radius_to_path` and `clip_path_for_rect_alike_item`. The pool
  is bounded to 8 paths so nested clips cannot grow it without limit. **Reverted** from the
  tree, see below.

## Measured (FemtoVG under Xvfb/llvmpipe, `--frames 8`)

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
# baseline / patched, same command
just frames-xvfb-mode rects 3000 8      # or: perf_test --mode rects --count 3000 --frames 8
perf_test --mode rects --count 3000 --frames 6 --alloc-profile 3   # site ranking
```

## Correctness

- `cargo test --lib` in `../../../femtovg`: 61 passed.
- `cargo test -p i-slint-renderer-femtovg` (with the root workspace temporarily patched to the
  local femtovg): compiles and runs (the crate has no unit tests of its own).
- Pixel comparison of the last frame, before vs after (`--snapshot`, added to perf_test for
  this):
  - `rects` scene: **byte-identical** PPM.
  - `list` and `text` scenes: differ by 132 and 1 158 bytes of 3 302 400 - but two runs of the
    *same* binary differ by 300 and 1 395 bytes, so the scenes are nondeterministic (text
    cursor blink / llvmpipe glyph timing) and the patch is well inside that noise.

## Why it is not applied to the tree

`slint_scratch_path.diff` needs `femtovg::Path::clear()`, which exists only in the local
checkout. The repository's `examples/`, `demos/` and `tests/` are separate workspaces that
build `i-slint-renderer-femtovg` against **crates.io** femtovg, so applying the renderer patch
breaks them until `Path::clear()` is released (or every workspace gets a
`[patch.crates-io] femtovg`). `Path::clear()` itself is additive and stays in the checkout -
it is the piece to upstream first.

To try it: apply `slint_scratch_path.diff` and add to the root `Cargo.toml`
`[patch.crates-io] femtovg = { path = "femtovg" }` (perf_test already has it).

## Next step, not attempted: the tessellation cache (another ~2 allocations per primitive)

The site ranking after this patch:

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
would remove ~2 more allocations per primitive. The `Vertex`/`Drawable` vectors above are
femtovg's per-draw-call render buffers - another ~3 per primitive, and the same pooling idea
applies inside femtovg's renderer.
