# TEXTURE_CACHE_DRAIN_FULL_WALK

`internal/renderers/femtovg/images.rs`'s `TextureCache::drain()`, called once per frame from
`lib.rs`, unconditionally runs `HashMap::retain(...)` over the *entire* per-source GPU texture
cache every frame, checking `Rc::strong_count(cached_image) > 1` for every entry regardless of
how many images were actually touched, added, or evicted that frame - shape-identical to
candidate `057.FEMTOVG_TILED_9SLICE_TEXTURE_PER_FRAME`'s "rebuild/walk every frame regardless of
change" defect in the same file, which measured an -87.1% win. Flagged in `IDEAS.md` as an
unmeasured lead from the 2026-08-23 femtovg/software sweep.

## What was tried

Added `inserted_since_last_drain: bool` and `frames_since_last_drain: u32` fields to
`TextureCache`. `lookup_image_in_cache_or_create`'s vacant-entry (insert) branch sets
`inserted_since_last_drain = true`. `drain()` skips the `retain()` walk entirely (just bumping a
frame counter) when nothing was inserted since the last real walk *and* fewer than
`FORCE_DRAIN_EVERY_N_FRAMES` (20) frames have passed - a periodic full sweep still runs every 20
frames regardless, so an entry that becomes collectible without a corresponding insert (e.g. an
item destroyed, or a dependency invalidated, dropping the external `Rc<Texture>` held elsewhere by
`ItemGraphicsCache`) is still eventually reclaimed rather than leaked. This is a bounded-staleness
heuristic, not a fully accurate one: unlike `057`'s `TiledTextureCache` (which tracks "used this
frame" via an explicit lookup on every draw), this cache's *hit* path
(`get_or_update_cache_entry` returning an already-valid `ItemGraphicsCache` entry) does not call
back into `TextureCache` at all for a steadily-drawn, unchanged image - so there is no local signal
available to detect "this entry just became garbage" without either the periodic sweep or a
cross-module invalidation channel from `ItemGraphicsCache` (out of scope for this candidate - see
"What's not done" below).

Reused `perf_test`'s `images` scene, extended with a new `--image-unique` flag +
`generator::make_unique_gallery_images` (see `perf_test_fixes/`). This was necessary, not
optional: the scene's existing `make_image()`-generated sources (used by `--image-distinct`) are
in-memory `SharedPixelBuffer`s, which get `ImageCacheKey::Invalid` ("no image cache key can be
created for ... programmatically created images", `internal/core/graphics/image.rs`) - so they
**never populate `TextureCache` at all**, recreating a texture from scratch every frame regardless
of any cache (confirmed directly: a temporary instrumentation build showed `TextureCache::len()`
staying `0` throughout a run with `--image-distinct`/the old `--image-unique` prototype). Instead
of file-less procedural buffers, `--image-unique` now writes `count` distinct tiny PNGs to
`assets/gallery/` (cached across runs, deterministic content) and loads each via
`Image::load_from_path`, which does get a real, persistent `ImageCacheKey::Path` - the only way to
actually reach and grow the cache `drain()` walks. Also confirmed (same instrumentation) that with
the *default* `--image-cell 68px`, off-screen items in a large, non-scrolling grid never reach
`draw_image_impl` at all (`TextureCache` stayed at a constant ~216 entries from `--count 300` up to
`--count 10000`) - reaching a genuinely large cache size needed a tiny `--image-cell`/`--image-size`
so thousands of distinct images are simultaneously *visible*, not just present in the item tree.

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 | notes |
|---|---|---|---|
| baseline, `--count 2000 --image-unique --image-size 4 --image-cell 6 --frames 10` | 41.42 ms | 18.18 ms | later frames, cache size 2000 |
| patched, same | 41.29 ms (-0.3%) | 17.78 ms (-2.2%) | |
| baseline, `--count 6000`, same flags | 49.25 ms | 45.44 ms | cache size 6000 |
| patched, same | 49.67 ms (+0.9%) | 47.74 ms (+5.1%) | high spread both sides (42-103 ms) |
| baseline, `--count 12000`, same flags | - | 87.19 ms | cache size 12000 |
| patched, same | - | 86.53 ms (-0.8%) | |

All numbers: `RUNS=9`, `paired_femtovg_later_frames.sh`, median of 9 alternating runs each side,
`--renderer femtovg`, `xvfb-run`. Machine: 24 cores, `uptime` load ~2.6-5.8 at measurement time
(below core count).

A separate temporary-instrumentation measurement (`Instant` around `drain()` itself, not part of
the saved patch) found `drain()`'s own cost at steady state: ~22-27 µs at 2000 entries, ~1-2.5 µs
at 216 entries (the default-cell, off-screen-pruned case) - a few tenths of a percent of a
4-17 ms frame even at the largest cache size this harness could construct.

## Result

No measurable win at any tested cache size (2000/6000/12000 entries): every delta (-2.2% to
+5.1%) falls inside the run-to-run spread seen on the *unpatched* binary alone (e.g. baseline's
own 6000-entry runs ranged 42.0-75.4 ms multi-core), i.e. within noise, not a real, reproducible
effect in either direction. This tracks the direct cost measurement: `TextureCache::drain()`
itself costs tens of microseconds even at the largest cache this harness could build (12000
entries would extrapolate to roughly 130-160 µs at the same per-entry rate), against multi-ms
frame times dominated by the O(N) *drawing* work for the same N images - unlike candidate 057's
target (a real GPU texture create+blit+destroy cycle per frame, independent of and much larger
than `retain()`'s per-entry cost), `Rc::strong_count` plus a short-circuited `.size()` call is
cheap enough that a full walk never becomes a disproportionate share of the frame even at the
largest entry counts this harness could reach.

## Does it change behavior/output?

- **Bit-exact?** Yes. `cmp`-identical PPM snapshots between baseline and patched binaries at
  `--count 300 --image-unique --frames 40` (40 frames, twice past the new 20-frame forced-sweep
  threshold) and again with `--image-rescale --resize-step 30` added (exercises per-frame texture
  re-creation/eviction as the target size changes). Noise floor separately confirmed as exactly 0
  differing pixels across 3 same-binary runs at the same settings.
- **Is this a breaking change?** No. Internal-only change to `i-slint-renderer-femtovg`'s private
  `TextureCache`; no public API change.
- **Non-pixel behavior change**: an evicted texture's GPU memory can now be held for up to 20
  frames longer than before, in the specific case where no *new* distinct image was inserted into
  the cache during that window (see "What was tried" - the periodic sweep bounds this to at most
  ~20 frames, roughly a third of a second at 60 FPS, never indefinitely). Not observed to affect
  peak resident memory in any measurement taken (`resident memory` line, all runs above) but not
  exhaustively stress-tested for a fast-churning-then-idle scene.
- Project test suite: `i-slint-renderer-femtovg` has no unit tests to run, before or after (`cargo
  test -p i-slint-renderer-femtovg`: `0 passed; 0 failed` both baseline and patched). No other
  crate's tests touch this code path.

## Why the result looks the way it does

The premise ("O(total cached textures) walk every frame") is accurate in shape, but the per-entry
cost (`Rc::strong_count` plus a short-circuited `Option::is_none()`) is orders of magnitude
cheaper than candidate 057's per-frame GPU texture create+blit+destroy cycle, and the achievable
cache size in any real scene is bounded by *simultaneously visible* distinct images (confirmed:
off-screen items in a large non-scrolling grid never reach `draw_image_impl` at all, so the cache
never grows past what's on screen) - itself already bounded by the O(N) cost of actually drawing
those N images every frame. The two costs scale together, so `drain()` stays a small, roughly
constant fraction of the frame rather than growing to dominate it. The candidate's own backlog
note ("needs a many-distinct-images bench... to size before treating as more than a lead") turned
out to be the right caution: sizing it is exactly what shows the win isn't there in practice, at
least not in any scene this harness (or, per the visibility-pruning finding above, a real Slint
UI within the same design) can construct.

## What's not done / open questions

- A cross-module invalidation channel (`ItemGraphicsCache` notifying `TextureCache` directly when
  an entry is dropped/replaced) would let `drain()` skip the walk soundly rather than
  heuristically, but touches `internal/core/item_rendering.rs` and every renderer that uses
  `ItemGraphicsCache`, not just femtovg - out of scope for a single candidate and not attempted.
- Not tested: a scene that inserts a new distinct image on some frames and destroys an item
  (without inserting) on others, repeatedly, to specifically probe the 20-frame staleness bound's
  effect on peak GPU memory - the harness's `images` scene has no per-frame item-removal modifier.
- **New harness capability found while building the bench for this candidate, worth flagging as
  its own lead**: `make_image()`-sourced images (used by the existing `--image-distinct`) get
  `ImageCacheKey::Invalid` and so bypass `TextureCache` entirely, recreating their GPU texture from
  scratch every single frame regardless of the "1 shared"/"64 distinct" setting. This is a real,
  likely much larger inefficiency than the one this candidate investigated, but it is a
  procedurally-generated-image-specific limitation (by-design per `ImageCacheKey::new`'s doc
  comment, not obviously a bug) - added to `IDEAS.md` rather than investigated further here, since
  it's a different code path (the uncached `.or_else()` fallback in `draw_image_impl`/
  `draw_image_direct`, not `TextureCache::drain()`).
