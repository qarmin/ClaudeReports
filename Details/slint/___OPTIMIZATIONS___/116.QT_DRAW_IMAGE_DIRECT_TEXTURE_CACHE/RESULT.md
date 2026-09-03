# 116 - Qt backend `draw_image_direct` per-source texture cache

`QtItemRenderer::draw_image_direct` (`internal/backends/qt/qt_window.rs`) called `image_to_pixmap`
(full resvg SVG rasterization + `QImage`/`QPixmap` construction) unconditionally on every call,
with zero caching - candidate 115's audit found this while widening its check (scoped to native
widgets) to every `image_to_pixmap` call site in the crate, noting femtovg's `TextureCacheKey`/
`texture_cache` and Skia's `cached_image::as_skia_image` both give their own `draw_image_direct`
this treatment already.

**Correction to this idea's own framing found while implementing it:** `draw_image_direct` does
*not* back the general `Image { source: ... }` element's rendering path - that path is
`draw_image`/`draw_image_impl`, which already has its own per-item `ItemCache<QPixmap>` (keyed by
`ItemRc`, see `self.cache` in `qt_window.rs`). `draw_image_direct`'s only call site in the whole
crate is `i_slint_core::window::WindowInner::render_drag_image_overlay` (`internal/core/window.rs`),
which paints a `DragArea`'s `drag-image` under the cursor while a drag is in flight - called once
per repaint during an active drag, not once per `Image` element per frame. This narrows the
practical impact (at most one drag in flight at a time, so at most one image needs caching, not
"every Image element on screen") but the underlying defect - and the fix - are exactly as
originally described.

Because there is only ever one image in flight through this call site, the fix is a **single-slot
cache** (`RefCell<Option<(ImageCacheKey, Option<Size2D<u32>>, QPixmap)>>` on `QtWindow`, threaded
into `QtItemRenderer`), not a multi-entry `HashMap` like femtovg's `TextureCache` - a single slot
gets a full hit rate for the one caller that exists, without needing femtovg's per-frame
`drain()`-based eviction (which exists there specifically to bound a multi-entry map that femtovg's
own `draw_image`/`draw_image_direct` split doesn't need for this call site). The cache key mirrors
femtovg's `TextureCacheKey`: `ImageCacheKey::new(image)` (source identity) plus the SVG target size
the image was rasterized at (`None` for non-scalable sources, since those are always rasterized at
native size regardless of draw-time target size, both before and after this patch).

## What was tried

Harness: a new standalone probe (`bench/qt_draw_image_direct_probe.rs`,
`qt-draw-image-direct-probe` crate) - no existing project benchmark reaches the Qt backend's
`draw_image_direct` (`perf_test`'s "qt" feature only makes the whole app use the Qt
backend/style, and doesn't drive a `DragArea` drag). It calls the **real, unmodified**
`image_to_pixmap` and the new `cached_image_to_pixmap` directly (exported behind a temporary
`bench-probe` cargo feature, `bench/bench_probe.patch`, never shipped) in two modes:

- `uncached`: calls `image_to_pixmap` on every iteration - what every `draw_image_direct` call
  does today.
- `cached`: calls `cached_image_to_pixmap` with a persistent single-slot cache reused across
  iterations, exactly what the patched `draw_image_direct` does - not a re-implementation of the
  cache logic in the probe, the literal production function.

This mirrors candidate 113's probe design for the analogous Qt native-widget-icon fix, improved to
call the real cache function instead of re-implementing its comparison in the probe. Correctness is
checked every iteration: the returned pixmap's `(width, height)` must be stable and identical to
iteration 0. `ensure_initialized()` (Qt's lazy `QApplication` bring-up) is invoked explicitly by the
probe (`bench_probe_init()`) since it bypasses the normal widget `init()`/`render()` cpp! call
sites. `QT_QPA_PLATFORM=offscreen`, no display needed.

Asset: `testdata/shapes.svg` (same 400-path file candidates 041/113 used), rasterized at 24px,
48px, and 128px target sizes as representative drag-image sizes. This binary is single-threaded,
so "single-core" (`taskset -c 3`) and "multi-core" (unpinned) are expected to and do coincide.
`uptime` load average 2.1-2.6 on 24 cores throughout (idle sandbox).

| target size | mode | single-core p50 (run 1) | single-core p50 (run 2) | multi-core p50 (unpinned) |
|---|---|---|---|---|
| 24px | uncached | 0.370 ms | 0.370 ms | 0.363 ms |
| 24px | cached | 0.000 ms | 0.000 ms | 0.000 ms |
| 48px | uncached | 0.537 ms | 0.531 ms | 0.537 ms |
| 48px | cached | 0.000 ms | 0.000 ms | 0.000 ms |
| 128px | uncached | 1.073 ms | 1.084 ms | 0.865 ms |
| 128px | cached | 0.000 ms | 0.000 ms | 0.000 ms |

(`min` tracks `p50` within a few microseconds at every size/mode across both single-core repeat
runs.) `p95`/`max` occasionally spike into single digits of milliseconds on the uncached path
(scheduler/page-fault jitter from this shared sandbox), never visible on `p50`/`min`.

## Result

`cached` mode's cost is below this timer's microsecond resolution (`0.000 ms`) for every iteration
after the first, at every size tried, while `uncached` costs a real, size-scaling 0.37-1.08 ms
*every single call* - i.e. every repaint while a drag is in flight. This is the same shape of
result candidate 113 found for the native-widget-icon case: not a marginal win, effectively all of
the per-call cost is eliminated on the (now guaranteed, single-slot) cache-hit path. The practical
benefit is bounded to however many repaints happen during an active drag-and-drop gesture with a
`drag-image` set (continuous, once per mouse-move-triggered repaint, for as long as the drag lasts)
rather than every frame of general UI rendering - see the correction above.

## Does it change behavior/output?

- **Bit-exact?** Yes, by construction, not just by testing: a cache hit returns
  `cached_pixmap.clone()` - Qt's `QPixmap` copy constructor is implicit-sharing (a cheap handle/
  refcount copy, not a new render) - of the exact same `QPixmap` object `image_to_pixmap` produced
  on the cache-miss call that created it, never a re-derived approximation. The cache key
  (`ImageCacheKey::new(image)` + the SVG target size) captures exactly the two inputs
  `image_to_pixmap`'s output depends on, so any change to the source image or its rasterization
  target size (including a scale-factor change, which changes the target size) correctly
  invalidates the slot and forces a fresh rasterization. The probe's own per-iteration size-stability
  check never failed at any tested size/mode.
- **Is this a breaking change?** No public API changes - `QtWindow`/`QtItemRenderer` are internal
  (`i-slint-backend-qt` crate) types, not part of any public Slint API. The only new state is a
  private `RefCell` field on `QtWindow`.
- Project test suite (`cargo test -p i-slint-backend-qt --features rtti`, `QT_QPA_PLATFORM=offscreen`,
  real Qt 6.5.3 linked via `aqtinstall` into `tmp/Qt`, gitignored): **0 unit tests, 1/1 doctest
  passed** before (plain `master`, `cdb6c43f7`) and after (patched, without the `bench-probe`
  feature) - identical, no new failures. (Without `--features rtti` the doctest fails to build on
  *both* baseline and patched trees - the same pre-existing, patch-unrelated gap candidate 113
  already noted for this crate's default-feature doctest.)

## Why the result looks the way it does

Same underlying cause as candidate 113's native-widget-icon fix: `image_to_pixmap` does a full
resvg SVG rasterization (`ImageInner::render_to_buffer`) plus a Qt `QImage`/`QPixmap` construction,
with no memoization at all on the unpatched tree - every call pays the full cost again even when
nothing about the image or its target size changed. A single-slot cache turns repeated calls with
an unchanged `(image, target size)` pair into a cheap key comparison plus an implicit-sharing
pointer copy, which is why the eliminated cost shows up as "immeasurably fast" rather than "a bit
faster" on the hit path - there's no per-call work left to measure once the size-24/48/128 warmup
call has happened once.

## What's not done / open questions

- `render_drag_image_overlay` being `draw_image_direct`'s only call site means this fix's
  real-world benefit is scoped to drag-and-drop-with-a-drag-image sessions, not general `Image`
  element rendering - see the correction in the opening section. If a future change adds another
  `draw_image_direct` call site that draws more than one distinct image per frame (unlike today's
  single-caller reality), the single-slot cache's hit rate would drop accordingly; a multi-entry
  keyed cache (mirroring femtovg's `TextureCache` more closely, including its `drain()`-style
  eviction) would then be worth revisiting - flagged in `IDEAS.md` as a follow-up condition, not
  implemented speculatively here since today's single caller doesn't need it.
- Not independently verified end-to-end through a live `QApplication` drag gesture (moving the
  mouse with a button pressed over a real `DragArea`) - the probe calls the same functions
  `draw_image_direct` calls directly, per candidate 113's precedent, rather than standing up a full
  synthetic drag event sequence.
- The `bench-probe` cargo feature and its `qt_window.rs`/`lib.rs` exports are measurement-only
  scaffolding (`bench/bench_probe.patch`), never intended to ship - kept separate from
  `patch.diff` (the actual proposed fix) so `patch.diff` alone is postable as-is.
