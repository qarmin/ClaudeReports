# FEMTOVG_TILED_9SLICE_TEXTURE_PER_FRAME

`internal/renderers/femtovg/itemrenderer.rs`'s `draw_image_impl`, when an `Image` uses
`ImageTiling::Repeat`/`Round` together with a source crop smaller than the full source image
(the common case for 9-slice edges/center, but also reachable by a plain `Image` with
`horizontal-tiling`/`vertical-tiling` set and `source-clip-*` set), builds a clipped/tiled copy of
the source by calling `create_image_empty`, switching the femtovg render target, blitting into it,
and scheduling it for deletion after the next flush - unconditionally, on every single frame,
regardless of whether the source image, tile geometry, or clip changed. This never goes through
the outer `graphics_cache`/`texture_cache`, so every frame pays a full GPU texture
create+blit+destroy cycle per tiled image, even for a completely static scene. Flagged by a prior
femtovg/software discovery sweep as "likely the strongest finding" among several allocation- and
GPU-resource-per-frame findings (see `IDEAS.md`'s now-removed entry); same defect class as
candidate `041.SVG_RASTERIZED_EVERY_FRAME` but on the GPU-texture side.

## What was tried

Added a second, content-addressed cache (`TiledTextureCache`, `internal/renderers/femtovg/images.rs`)
keyed on `(source femtovg::ImageId, clip_rect, texture_width, texture_height, ImageRendering)` -
the exact inputs that determine the clipped texture's contents. `draw_image_impl` now looks the
texture up in this cache instead of unconditionally creating and blitting a new one; a cache miss
still does the same `create_image_empty` + render-target-switch + blit as before, a hit just
reuses the existing `femtovg::ImageId`. Entries are retained across frames by marking them "used"
on lookup and pruning anything not looked up that frame in `TiledTextureCache::drain()`, called
once per frame right after `TextureCache::drain()` (same place, same idea: nothing outside the
cache holds a persistent `Rc` to these textures, unlike the base per-source `TextureCache`, so
retention has to be tracked explicitly rather than via `Rc::strong_count`). The previous
`textures_to_delete_after_flush` scheduling for these textures is removed - a cached texture is
kept alive by the cache itself and only actually deleted (via `Texture`'s `Drop`) once
`drain()` evicts it, which already happens at the same post-flush point the old
`textures_to_delete_after_flush` mechanism required for safety.

`perf_test`'s `images` scene had no way to reach `Repeat`/`Round` tiling with a source crop before
this candidate (plain tiling without a crop never hits the expensive branch - the clip rect equals
the full source, so `fit.clip_rect.size.cast() != orig_size` is false). Added a `tiling: bool`
property to `ImageModePage` (`perf_test/ui/image-mode.slint`) and a `--image-tiling` CLI flag
(`perf_test/src/backend.rs`, `bench.rs`, `main.rs`, `perf_test/ui/main.slint`) that sets
`horizontal-tiling`/`vertical-tiling: repeat` and a source crop of `(8, 8, 40, 32)` px on every
image in the grid (source images are 64x64 by default, well inside that crop). See `../README.md`
for this as a permanent harness addition.

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 | notes |
|---|---|---|---|
| baseline, `--mode images --count 200 --image-tiling` | 14.01 ms | 28.98 ms | later frames (steady state) |
| patched, same | 3.59 ms | 3.74 ms | |
| baseline, `--count 1000 --image-tiling` | - | 31.23 ms | later frames, multi-core only |
| patched, same | - | 4.18 ms | |
| baseline, `--count 1000`, no `--image-tiling` | - | 4.51 ms | regression check on the untouched (non-tiled) path |
| patched, same | - | 4.55 ms | |
| baseline, `--count 200 --image-tiling`, first frame | - | 47.87 ms | both sides build the texture fresh on frame 0 |
| patched, same, first frame | - | 47.97 ms | |

All numbers: `RUNS=9`, `paired_femtovg_later_frames.sh`/`paired_femtovg_first_frame.sh` (new,
adapted from the existing `paired_skia_*` scripts - femtovg has no headless `--bench` path either,
so the comparison runs `--frames` under `xvfb-run` the same way), median of 9 alternating runs each
side, `--renderer femtovg`. Machine: 24 cores, `uptime` load 0.7 at measurement time.

## Result

Real, and not close to the noise floor: **-87.1%** later-frame time at `--count 200`
(28.98 ms -> 3.74 ms, multi-core), **-74.4%** single-core pinned (14.01 ms -> 3.59 ms), **-86.6%**
at `--count 1000` (31.23 ms -> 4.18 ms, multi-core). The first frame is unaffected either way
(+0.2%, within noise) since both sides build the texture from scratch on the very first render -
the win is entirely in steady state, which is also where a real static or slowly-scrolling scene
spends almost all of its frames. The non-tiled path (no `--image-tiling`) is a wash (+0.9%,
within noise), confirming the change is additive and doesn't touch the common (non-tiling)
`draw_image_impl` path.

## Does it change behavior/output?

- **Bit-exact?** Yes. `cmp`-identical (not just outside-noise-floor identical) PPM snapshots
  between baseline and patched binaries across `--image-tiling`, `--image-tiling --image-distinct`,
  and `--image-tiling --image-rescale`, `--count 300`, `--frames 6`, under `xvfb-run`. Additionally
  checked a 0-pixel noise floor and 0-pixel raw difference across 3 same-binary runs each at
  `--count 200 --image-tiling --frames 10` (script in `../tools/`, adapted from `pixelcheck.sh` for
  `--frames`/`xvfb-run` since femtovg has no headless path).
- **Is this a breaking change?** No. Internal-only change to `i-slint-renderer-femtovg`; no public
  API, file format, or documented behavior changes.
- Project test suite: `i-slint-renderer-femtovg` has no unit tests to run. No other crate's tests
  touch this code path.

## Why the result looks the way it does

`create_image_empty` + `set_render_target` + `fill_path` (the blit) + the scheduled delete is a
real GPU round-trip (texture allocation, a render-target switch, a draw call, and eventually a
deletion), not just a CPU-side allocation - that's why the win is far larger than a typical
"skip one small alloc" fix and closer in shape to candidate 041's SVG-rasterization fix (both
replace a full GPU/CPU resource rebuild with a cache hit). The fix's mechanism (a real,
content-addressed cache keyed on the exact inputs that determine the texture's contents,
lifecycle-managed the same way `TextureCache` already is - drained once per frame, nothing else
holds a persistent reference) directly mirrors what the candidate description asked for and what
`041`/the outer `graphics_cache` already do for other GPU resources.

## What's not done / open questions

- Not tested: a scene where the tiled image's geometry or source actually changes every frame
  (e.g. an animated resize or a per-frame source swap) - `perf_test`'s `images` scene has no
  `--mutate`/`--resize-step`/`--animate` support yet for tiled images specifically. In that case
  the new cache would thrash (miss every frame) and fall back to roughly the baseline cost plus a
  hash lookup - not a regression, but not the measured win either. Worth a follow-up if a workload
  like that turns out to matter (added to `IDEAS.md`).
- 9-slice images specifically (`ImageInner::NineSlice`, up to 9 `fit`s per item, several of which
  can each need their own cached tiled texture) were not separately exercised - `perf_test` has no
  9-slice asset/scene. The code path is shared with plain `Repeat`/`Round` tiling (same cache, same
  key derivation per `fit`), so the mechanism should generalize, but this was not independently
  measured.
- The cache has no explicit size bound (unlike, say, `IMAGE_CACHE`'s bound flagged in candidate
  038/052) - it is bounded in practice by "however many distinct (source, clip, size, rendering)
  combinations are drawn in a given frame," which tracks the number of on-screen tiled/9-slice
  images, not an unbounded growth source, but this wasn't stress-tested with a very large distinct
  count.
