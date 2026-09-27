# Optimization wins (Strong win / Small win only)

Code changes to Slint (or a vendored dependency) only - app-level `.slint`-authoring findings with
no engine change (001, 013, 044) are excluded, see `SUMMARY.md`'s header for why.

Filtered from `SUMMARY.md` - see that file for every candidate tried (including null results and
regressions), the full methodology behind each number, and this repo's own candidate folders. No
Link column here on purpose - point readers at `___OPTIMIZATIONS___/SUMMARY.md` in prose instead.
**Fix lines** is `+A/-D` (added/deleted) from that candidate's own `patch.diff`, via `git apply
--numstat patch.diff` against the pinned base commit.

| # | Impact | Candidate | Fix lines | Result |
|---|---|---|---|---|
| 3 | Strong win | `TextInput` shapes the whole document, one parley `Layout` per paragraph, instead of one per visible line | +890/-131 | `no-wrap`: RSS 540 -> 39MB software / 646 -> 130MB GPU, 2.01 -> 1.73s, resize 76 -> 5.6ms |
| 6 | Small win | femtovg allocates a fresh `Path` per primitive per frame instead of reusing one | +63/-3 | -7.2% allocations/frame; needs a femtovg release for `Path::clear()` |
| 8 | Small win | Duplicate dependency registrations are never skipped (`front_is` check missing) | +122/-0 | -12% allocations/frame on `scroll` (pooling absorbs most of the gain elsewhere) |
| 9 | Small win | Gradient stops are collected into a fresh `Vec` per primitive per frame | +36/-29 | -1 allocation per gradient primitive |
| 10 | Small win | Gradients are tessellated twice because the bbox is measured with the transform reset | +74/-46 | -5 allocations per gradient primitive, -3.7% frame |
| 11 | Strong win | femtovg's tessellation buffers (`PathCache::rebuild`) are reallocated instead of reused | +53/-9 | -30% allocations/frame on `rects`, -35% on `list` |
| 14 | Strong win | Every tracked property read allocates a dependency node, every frame | +211/-32 | Pooling: `scroll` 100k -8.0% frame; `bare` -23.7% allocations/frame |
| 19 | Small win | Resolving a text selection walks the whole document instead of just the visible band | +30/-18 | Binary search on visible band: +1.09ms -> +0.43ms flat at 400000 lines |
| 23 | Strong win | Glyphs are never culled horizontally, only vertically, in the software renderer | +58/-0 | -39.8% single-core / -39.7% multi-core (paired, 9/9) |
| 26 | Strong win | The shaped-text cache never evicts | +88/-1 | 1024-entry LRU-ish bound; RSS -67% at 60000 rows (917->305MB) |
| 29 | Small win | `SortModel` makes appending O(model) via a needless no-op index-adjustment scan | +57/-4 | Skip the scan on append: -59% |
| 35 | Strong win | A culled item's geometry is still read for nothing | +50/-16 | -12% on `scroll` 100000 `bare`, -12.4% on 20000 layout rows |
| 36 | Small win | Renderer state is pushed/popped for every item even when unnecessary | +21/-8 | -8.2% on `scroll` 100000 `bare` (on top of candidate 035) |
| 41 | Strong win | An SVG is re-rasterized on every repaint of every item instead of cached | +28/-1 | Per-frame cache on `SceneBuilder`: -84.9% |
| 50 | Strong win | Candidate 049's per-frame subtree-bounds pass recomputes from scratch instead of incrementally | +190/-4 | Scrolling -10 to -27%, `bare` -10 to -15%, `rects` -4.9% |
| 51 | Strong win | Hit-testing descends blindly instead of reusing the subtree-bounds cache to prune | +242/-8 | `layout` -30.6%/-29.7%, `touch` -32.1%, `clipped` -24.5% |
| 52 | Strong win | `IMAGE_CACHE`'s 5MB bound evicts far too aggressively for real images | +7/-2 | A ~5.9MB image reload: ~18-29ms -> ~0.00-0.02ms |
| 53 | Strong win | `IdLookup` walks the full element tree on every plain-property identifier resolution | +101/-20 | Memoized lookup: 1.5x at 4000 elements, 2.05x at 8000 |
| 55 | Strong win | `Window::move_focus`'s cycle detection is O(n^2) instead of an O(1) start-item comparison | +8/-4 | 2.3x at N=2000 up to 62.8x at N=100000 (~1.01s -> ~16ms) |
| 56 | Strong win | Skia converts a shared image source to a texture once per item instead of once per source | +75/-10 | Realistic `--count 100`: -79.4%; later frames -78.2% at count 2000 |
| 57 | Strong win | femtovg's tiled/9-slice clip path rebuilds a GPU texture every frame instead of caching it | +141/-40 | Later frames -87.1% multi-core, -74.4% single-core |
| 62 | Strong win | `Image::from_rgba8` stores non-premultiplied alpha, forcing a per-pixel multiply on every draw | +8/-1 | Later frames -41.1% to -51.5% across 3 size/count combos |
| 66 | Small win | Skia's per-frame dirty-region clip always builds a `Path`, even for the common single-rect case | +31/-12 | ~1-2.6% faster with `clip_rect()` fast path |
| 68 | Strong win | Skia's `draw_*` calls have no alpha-gate before opacity-0 items | +23/-0 | -30.8%/-32.6% single-core, -30.4%/-42.5% multi-core on the offscreen-layer path |
| 70 | Strong win | `Text::layout_info`'s `Vertical`/`NoWrap` branch shapes through parley just to read `.height` | +17/-1 | First frame -82.1% at 20000 rows; RSS -69% at 100000 rows |
| 73 | Small win | The per-item geometry tracker is boxed instead of living in a chunked, never-reallocating arena | +244/-24 | `layout` static -2.9%/-3.3%, `bare` scrolling -7.4%/-15.2% |
| 76 | Strong win | `SortModelInner::row_removed` does one O(n) `Vec::remove` per removed row, making batched removal O(n x count) | +17/-15 | Up to ~157x at count=5000 (batched compaction pass instead) |
| 77 | Small win | Focus-visibility tracker setup runs for every rejected `move_focus` candidate, not just accepted ones | +1/-1 | ~25-29% reduction isolated on top of candidate 055's fix |
| 79 | Strong win | femtovg's `visit_clip` always renders a rounded clip's children into an offscreen layer, even when content never reaches the rounded corners | +50/-2 | count=20000: -22.0%/-24.6% single-core, -23.3% multi-core |
| 80 | Small win | `chunked_arena::CHUNK_SIZE` was never swept for the scrolling scenes it's tuned against | +244/-24 | `4096` wins both scrolling scenes: `layout` -3.2%/-2.8%, `flat` -2.9%/-4.7% |
| 81 | Small win | `IMAGE_CACHE` has no dedup for a live `Image` past the byte bound | +56/-8 | `Svg`-only weak side table: reload 8.3-9.4ms -> 0.01-0.03ms while held |
| 82 | Small win | `free_graphics_resources` forces a full-screen refresh on every item-tree teardown | +79/-9 | Popup close-frame render: ~33% faster (2.951ms -> 1.978ms) |
| 85 | Small win | The software renderer's font-routing check duplicates `sharedparley`'s own `fontique` query | +22/-0 | Wide-viewport `list`: -5.2%/-5.1% single-core, -4.0%/-3.3% multi-core |
| 88 | Small win | Rust codegen's per-sub-component `ItemTree` methods always emit a `match`, even with 0 branches | +139/-56 | Generated Rust size -0.78%, consistent across fixture sizes |
| 90 | Small win | `SortModelInner::row_changed` has no hybrid O(1)/O(n) fallback, regressing large reorders | +54/-1 | Worst case regression eliminated; ~3.4-3.7x win kept on small/medium reorders |
| 91 | Strong win | Typing reshapes the whole document instead of incrementally diffing against the shaped-text cache | +256/-44 | 50000 lines: 2.41s/1.38s -> 90.1ms/86.9ms (~15-27x) |
| 98 | Small win | `TextInput`'s own `layout_info` lacks candidate 070's font-metrics height fast path | +15/-1 | -5.3% to -5.9% at 20000 rows, allocations -12.9% |
| 100 | Strong win | `ItemRc::absolute_clip_rect_and_geometry` is O(2^k) for k nested `clip: true` ancestors, read on every mouse/key event | +29/-9 | Up to ~104,000x at depth 22 (~150-300ms/move -> ~5.7us) |
| 103 | Small win | `is_visible_or_clipped_by_flickable` recomputes the same clip-rect/geometry value 2-3 times per call | +19/-16 | ~5-6% isolated on top of candidates 100+055 |
| 108 | Strong win | `draw_image_direct` (femtovg drag overlay) has no cache and re-uploads to the GPU every frame | +35/-2 | -65.4% at 2048px drag images; also fixed a pre-existing solid-black render bug |
| 109 | Strong win | `RepeaterTracker::reset` destroys and recreates every row instance on any model reset, even at unchanged row count | +8/-1 | `bare` -29.6%/-29.8% at 20000 rows, -26.1%/-25.9% at 60000 |
| 112 | Small win | Skia's `RendererSealed` never got candidate 051's subtree-bounds cache wired in | +249/-8 | `layout` -12.3%/-15.0%, `touch` -18.1% single-core |
| 113 | Strong win | Qt `NativeButton`/`NativeTab` re-rasterize their icon on every paint, uncached (GitHub #7511) | +35/-9 | Uncached 0.33-1.0ms/call -> cached ~0.000ms after the first call |
| 116 | Strong win | Qt's `draw_image_direct` (drag overlay) has no cache, unlike femtovg/Skia's equivalents | +42/-1 | Uncached 0.37-1.08ms/call -> cached ~0.000ms after the first call |
