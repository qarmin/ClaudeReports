# 41 - an SVG is re-rasterized on every repaint of every item

The software renderer's image draw ends in
`image_inner.render_to_buffer(Some(svg_target_size))` (`software/lib.rs:2415`) - **inside the draw
function**. For an `ImageInner::EmbeddedImage` that call clones a buffer and costs nothing. For an
`ImageInner::Svg` it runs `svg.render(size)`, a full resvg rasterization
(`core/graphics/image.rs:455`). Nothing caches the result.

So every `Image` item with an SVG source rasterizes the whole SVG again, every frame it is painted,
even when neither the source nor the target size has changed.

## The cost

`shapes.svg` (400 paths), `--repaint-buffer new` so every frame is a full repaint:

| scene | frame | allocations/frame |
| --- | --- | --- |
| 12 SVGs at 200px | **17.40 ms** | **24 018** |
| the same 12 as PNG at 200px | 0.80 ms | 6 |
| 30 SVGs at 200px | 35.40 ms | 48 031 |
| 1 SVG at 400px | 3.34 ms | 2 005 |

**An SVG costs 22x what the same drawing costs as a PNG**, and about 2 000 allocations per item per
frame. The per-item cost is flat (~1.4 ms), which is what "re-rasterized per item" predicts: twelve
items sharing one source rasterize it twelve times.

When nothing is repainted the cost disappears entirely (0.01 ms, 4 allocations with partial
rendering and a static scene), confirming this is per *draw* and not per frame.

## The prize

An experimental memo on `render_to_buffer`'s SVG arm, keyed by (source cache key, target size):

| scene | before | after | | paired |
| --- | --- | --- | --- | --- |
| 12 SVGs at 200px | 17.54 ms | **1.29 ms** | **-92.6%** | 7/7 |
| 30 SVGs at 200px | 34.96 ms | **2.13 ms** | **-93.9%** | 7/7 |
| 1 SVG at 400px | 3.34 ms | **0.67 ms** | -80.0% | 7/7 |

Allocations drop from **24 018 to 6** per frame. Rendering is unchanged (0 px outside the noise
floor).

At 1.29 ms the 12-SVG scene is finally within reach of the 0.80 ms the same drawing costs as a PNG -
the remaining gap is the larger RGBA source, which is candidate 39's territory.

## Applied: a per-frame cache, bounded by construction

`SceneBuilder` lives for exactly one frame, so a `HashMap` on it keyed by (source cache key, target
size) needs **no memory policy at all** - its bound is "the scalable sources drawn in this frame".
That is the safe half of the fix, and it is now applied (`per_frame_cache.patch`,
`tools/isolate.py off 41-svg-per-frame-cache`):

| scene | before | after | | paired |
| --- | --- | --- | --- | --- |
| 12 SVGs sharing one source at 200px | 17.29 ms | **2.61 ms** | **-84.9%** | 7/7 |
| 1 SVG at 400px, repainted each frame | 3.38 ms | 3.36 ms | -0.6% | 3/7 |
| 30 PNGs (control) | 1.22 ms | 1.21 ms | -0.8% | 4/7 |

Exactly the shape the mechanism predicts: N items sharing one source now rasterize it once instead
of N times, and a single item repainted every frame gains nothing because there is nothing to share
*within* the frame. Pixel-identical on four scenes including translucent images and the nine-slice
path; 374 `i-slint-core` tests pass.

## Remaining headroom - a cross-frame cache, and what it needs

The experimental cross-frame memo above is still worth **2.61 -> 1.29 ms** on the 12-item scene and
**3.34 -> 0.67 ms** on the single-item one. It is not applied, because unlike the per-frame version it
needs a memory policy, and that is a real decision rather than an oversight:

The experiment is a `HashMap` that clears itself wholesale past 32 entries. That is enough to size the
prize and wrong to ship:

- **it must be bounded by bytes, not entries.** A rasterized SVG is a full RGBA buffer; 32 of them at
  400x400 is 20 MB. Candidate 26 has the pattern (weigh entries, evict the least recently used half),
  and candidate 38 has the warning: a bound expressed as a constant in bytes silently stops caching
  anything larger than itself.
- **it must be keyed on the target size**, which the experiment does - an SVG animated in size gains
  nothing and would evict everything else, so a size-keyed cache needs the eviction policy to cope
  with a stream of misses rather than assume hits.
- **invalidation**: keying on `ImageCacheKey` ties the entry to the source, so a file reloaded under
  the same path would serve a stale rasterization. The existing image cache has the same property,
  so this is consistent rather than new - but it should be a decision, not an accident.

FemtoVG shows the shape a real answer takes: its `TextureCacheKey` already includes
`target_size_for_scalable_source` (`femtovg/images.rs:238`), and `TextureCache::drain()` keeps an entry
only while **something else still holds a reference** (`Rc::strong_count > 1`), dropping the rest after
each frame. The bound there is "in use", not a constant - which is exactly what candidate 38 said was
missing from `IMAGE_CACHE`. That pattern does not transfer directly, because the software renderer's
rasterized buffer is dropped as soon as it is drawn, so a cross-frame version needs a
retain-what-was-drawn-this-frame sweep instead.

## Scope

Measured on the software renderer only. `internal/renderers/femtovg/images.rs:212` calls the same
`render_to_buffer`, but the FemtoVG renderer keeps its own texture cache above it, so the same
conclusion must not be assumed there - it needs a GPU run to say anything.

## Reproduce

```sh
cargo run --profile rdebug --bin make_assets

RUNS=5 tools/matrix.sh \
  "svg|--mode images --count 12 --image-file shapes.svg  --image-cell 200 --repaint-buffer new" \
  "png|--mode images --count 12 --image-file medium.png  --image-cell 200 --repaint-buffer new"
```
