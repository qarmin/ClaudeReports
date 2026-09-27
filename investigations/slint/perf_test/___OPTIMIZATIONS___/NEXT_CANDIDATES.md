# Next candidates - a plan, not results

Nothing here is measured yet. Each entry says what the suspicion is, where in the code it lives,
how to isolate it, and what would make it a **null** result - written down first, so a disappointing
number is an answer rather than a temptation to keep looking.

Ordered by expected value within each group. Groups A-C are new ground; D is what earlier candidates
left standing.

---

## A. Software renderer

The renderer nobody profiled from the inside. Every frame number in this workspace passed through it,
so its internals are the denominator of all of them.

### A1. The scene item list is rebuilt and sorted every frame  -  **DONE: null (0.1-1.4%), `40_two_software_renderers/`**

`prepare_scene` (`software/lib.rs:1496`) builds a `Scene` from scratch each frame, and
`Scene::new` (`software/scene.rs:50-60`) does `items.sort_unstable_by(compare_scene_item)` followed
by a second partial sort by `z`. That is O(n log n) over **every drawn primitive**, plus the `Vec`s
in `SceneVectors` (`scene.rs:18-25`: textures, rounded rectangles, shared buffers, three kinds of
gradient) rebuilt per frame.

- **Measure**: `rects` at 300/3 000/30 000 *visible* primitives, and time with the sort replaced by
  a no-op (accepting wrong z-order) to size the sort alone; separately count the allocations the
  `SceneVectors` rebuild costs via the existing allocation counter.
- **Fix direction if real**: retain the `Vec`s across frames (`clear()` instead of drop), and skip
  the sort when the item order did not change - the item tree order is stable between frames.
- **Null if**: the sort is under ~2% at 30 000 primitives. Likely for small scenes; the question is
  the slope.

### A2. Every RGBA image pixel is premultiplied on every frame  -  **SIZED, small: `39_rgba_vs_rgb/`**

`fetch_blend_pixel` (`software/draw_functions.rs:186`) premultiplies **per pixel, per frame** in the
`TexturePixelFormat::Rgba` arm, while a `RgbaPremultiplied` format already exists in the same enum.
A static image redoes identical arithmetic for the whole of its life.

- **Measure**: draw a large `Rgba` image, compare against the same image converted once to
  `RgbaPremultiplied`; the delta is the whole prize. Needs B2's real assets to be meaningful.
- **Fix direction**: premultiply once on decode/upload and cache in that form.
- **Null if**: the multiply is hidden behind the memory traffic of reading the texture. Plausible -
  this loop is likely memory-bound, which is exactly why it must be measured and not assumed.

### A3. No opaque fast path for RGBA textures  -  **DONE: real gap (+80%), but the fast path is not the cause; `39_rgba_vs_rgb/`**

The `Rgb` arm has `if alpha == 0xff { *pix = from_rgb(..) }` - a straight write. The `Rgba` arm
always goes through `blend()`, even when every source pixel has `a = 255`, which is the common case
for a photo. Also `(data[pos+3] as u16 * alpha as u16) / 255` is a division per pixel.

- **Measure**: fully opaque PNG vs one with an alpha channel that is entirely 255 vs a genuinely
  translucent one; the first two should cost the same today and shouldn't.
- **Fix direction**: an "image is fully opaque" flag decided at decode time, dispatching to the copy
  path.
- **Null if**: <5% on a full-screen image blit.

### A4. The glyph cache is 1 MB and probably explains candidate 24  -  **DONE: null, see `24_.../CORRECTION.md`**

`GLYPH_CACHE` (`software/fonts/vectorfont.rs:65-72`) is a `clru` cache capped at **1 MB**, keyed on
(blob, index, pixel size, glyph id, variation coords). Candidate 24 measured **+57%** per frame with
128 distinct font sizes and concluded "the cost is per distinct size visible at once" without
identifying the mechanism. A 1 MB cap is a strong suspect: 128 sizes x ~80 glyphs at a few hundred
bytes each is several MB, so the cache would thrash.

- **Measure**: instrument the cache with hit/miss counters (as `TextLayoutCache` already has under
  `feature = "testing"`), then re-run candidate 24's `--font-sizes` sweep and plot misses. Then raise
  the cap to 8/16 MB and re-run.
- **This is the cheapest entry in the list**: it may turn an already-quantified 57% regression into
  a one-constant fix, and either way it *explains* candidate 24.
- **Null if**: misses stay flat as `--font-sizes` grows, which would mean the cost is rasterization
  of genuinely new glyphs and no cache size helps.

### A5. What a rounded corner actually costs  -  **DONE: it does scale, `25_.../CORRECTION.md`**

Candidate 25 measured a corner radius at **+15%** and explicitly recorded that the "cost grows with
radius" reading did not reproduce and was left unexplained. `RoundedRectangle` has its own entry in
`SceneVectors` and its own rasterization path.

- **Measure**: radius 0 vs 1 vs 8 vs 32 px at a fixed primitive count, with `--border 0` to remove
  the border confound, plus the same via `Path` to compare rasterizers.
- **Null if**: flat in radius, which is what candidate 25's re-check suggested - in which case the
  +15% is the *presence* of the rounded path, and the fix direction is a fast path for radius 0.

---

## B. Images - the harness has never touched a real one

`--mode images` (candidate 16) used `generator::make_image`, which builds an RGBA buffer
procedurally. So **decoding has never been measured, and neither has anything keyed on a file**.
Candidate 16's "all three flat, null" verdict is only about scaling an in-memory buffer.

### B0. Harness work this group needs  -  **DONE: `make_assets`, `perf_test/assets/`**

Add real assets under `perf_test/assets/` and generate them from a script so the repo stays
reproducible and no binary blobs are committed blind:

- PNG at 64x64, 512x512, 1920x1080, both opaque and with an alpha channel
- JPEG at the same sizes (a different decoder entirely)
- an SVG with a few hundred paths
- a 9-slice PNG for the `NineSlice` path

Then `--image-source <file>`, `--image-count`, `--image-reload` (drop and re-request each frame) and
`--image-fit <fill|contain|cover|tile>` on the existing `images` scene.

### B1. The image cache is 5 MB, so a full-HD image can never be cached  -  **DONE, confirmed: `38_image_cache_bound/`**

`IMAGE_CACHE` (`core/graphics/image/cache.rs:50-58`) is a `clru` cache with a **5 MB** total weight,
and the weight of an entry is its decoded byte size (`ImageWeightInBytes`). A 1920x1080 RGBA8 image
weighs **8.3 MB** - more than the entire cache. `clru` rejects an insert that exceeds capacity, and
the call site is `self.0.put_with_weight(cache_key, new_image.clone()).ok()` (`cache.rs:72`): **the
failure is discarded**.

If that reading is right, every use of a large image re-decodes it from disk, silently, forever.

- **Measure**: a scene showing one 1920x1080 PNG, counting decodes (instrument
  `lookup_image_in_cache_or_create`) across frames; then the same at 512x512 (1 MB, fits) as the
  control. Compare frame time and allocations.
- **Fix direction**: size the cache from the display resolution rather than a constant, and at
  minimum stop discarding the insert failure - an image too big to cache is worth knowing about.
- **Null if**: the decoded image is held alive by the item itself and the cache is only a
  deduplication aid, in which case nothing is re-decoded. **This is the first thing to check** and it
  decides whether the rest of B1 exists.

### B2. Decode cost per format, and whether it happens off the first frame  -  **DONE: `42_decode_cost/`**

Nothing measures PNG vs JPEG decode, or how much of a cold start is decoding.

- **Measure**: first-frame time and allocations for N images of each format and size; decode
  throughput in MB/s as the reportable number.
- **Interesting even if unfixable**: it sizes the "should decoding be async/threaded" question,
  which is an architectural answer, not a patch.

### B3. SVG  -  **DONE, and it is the biggest single finding so far: `41_svg_rasterized_every_frame/`**

`ImageInner::Svg` has its own weight function (`weight_in_bytes`) and resvg behind a feature flag.
An SVG is re-rasterized whenever the target size changes.

- **Measure**: an SVG at a fixed size across frames (should be cached) vs one animated in size
  (re-rasterized every frame), and against an equivalent PNG.
- **Expected**: a resizing SVG is very expensive. Worth knowing where the cliff is.

### B4. `--scale-factor` and images together

Candidate 31 covered a scale-factor change for text. Images are keyed differently, and a HiDPI window
means every image is drawn at a non-integer scale - the rescaling path candidate 16 called "O(destination
pixels)" - on every frame.

---

## C. Partial rendering configuration - the default is the one nobody measured  -  **DONE, see `37_repaint_buffer_mode/`**

`RepaintBufferType::NewBuffer` is `#[default]` (`core/partial_renderer.rs:337-340`). The harness uses
`ReusedBuffer` (`perf_test/src/bench.rs:293`). **Every number in this workspace is from the
non-default mode.**

And in `NewBuffer` mode `prepare_scene` calls
`software_renderer.partial_rendering_state.clear_cache()` **every frame**
(`software/lib.rs:1520-1527`), with the comment "to avoid unbounded growth of the partial rendering
cache". That drops every item's `CachedItemBoundingBoxAndTransform` *and* its `PropertyTracker`, then
rebuilds them all next frame.

- **Measure**: add `--repaint-buffer <new|reused|swapped>` to the harness and run the whole matrix in
  each mode. Allocations per frame are the tell: candidate 14 got them to 18-172 per frame in
  `ReusedBuffer`; if `NewBuffer` is rebuilding the cache, it will be orders of magnitude higher.
- **Why it matters beyond the number**: `NewBuffer` is what an embedded target without a retained
  framebuffer uses, which is much of the software renderer's actual audience. Candidates 14, 26, 32,
  33, 35 and 36 all touch structures that this mode throws away every frame - their benefit may be
  larger, smaller, or negative there, and none of it is known.
- **This is the highest-leverage item in the plan** because it does not need a fix to be worth doing:
  it tells us whether the last two sessions' conclusions generalize.

- **C2**: the comment says the cache is cleared to avoid *unbounded growth*. That is the same disease
  candidate 26 fixed in the text cache with a bound instead of a purge. If the growth is what forces
  the purge, bounding the partial-rendering cache would let `NewBuffer` keep its trackers.

---

## D. Still standing from earlier candidates

### D1. Subtree bounds (12, 13, 15, 32) - the largest single prize

Four traversals are O(total items) with no notion of what is on screen: rendering, layout,
hit-testing, dirty-region. Candidate 32 put an upper bound on the fourth alone: **-59%** of the frame
while scrolling 100 000 rows. Candidates 35 and 36 shaved the per-item constant; they did not change
the count of items visited.

The blocker is not measurement, it is design: candidate 33 showed the dirty-region walk must keep
registering a dependency that covers culled items, so subtree bounds need a *coarse* dependency per
subtree rather than none. That is a real design task and should be planned separately rather than
attempted opportunistically.

### D2. Typing re-shapes the whole document (20) - 882 ms per keystroke

Quantified, unfixed, and the worst absolute number in the workspace. Candidate 03's split makes an
incremental path tractable; the recorded blocker is that the cache entry is destroyed before it can be
diffed. Candidate 26 has since added `for_each`/`retain` and a sweep to that cache, so the machinery
for touching entries without going through the invalidation path now exists.

### D3. Prepending to a `VecModel` is O(model) (22)

+8.73 ms for 200 prepends into 200 000 rows. A deque-backed model, or a mapping layer, is an
app-level answer; whether Slint should ship one is a design question.

---

## E. Smaller, cheap to check

- **E1. `Model::row_data` per visible row per frame**  -  **DONE: null, `BATCH_SWEEP.md`** - does the repeater re-fetch and clone
  `SharedString`s every frame? Candidate 14 got allocations to ~18/frame, which argues no, but that
  was never checked directly.
- **E2. Timers**  -  **DONE: null, flat to 10 000 timers** - `maybe_activate_timers` (`core/timers.rs:255-295`) takes and rebuilds the active
  timer `Vec` on every tick. Fine for a handful of timers; measure with 1 000.
- **E3. Accessibility tree** - is it built eagerly, and does it walk the item tree? Never looked at.
- **E4. `--pointer-move` after 35/36**  -  **DONE: `15_.../RECHECK_AFTER_35_36.md`** - candidate 15's +38 ms per mouse move at 100 000 rows predates
  both fixes; the hit-test path may have moved.
- **E5. Compiler / build time** - `.slint` to Rust for a large UI. Different audience (developer
  iteration, not runtime), so it belongs in its own pass.

---

## Suggested order

1. ~~**C**~~ - **done** (candidate 37). The default mode is 1.7x *faster* above ~10 000 items and 8x
   slower below; candidate 35 turned out to be nearly inert in it. C2 (bounding the partial cache
   instead of purging it) is still open and now better motivated.
2. **A4** - a counter and a constant; may explain an already-measured 57%.
3. **B0 + B1** - the assets are needed for all of B, and B1 is a concrete, falsifiable claim about a
   silently discarded cache insert.
4. **A1**, then **A2/A3** once B0 makes them measurable on real data.
5. **A5**, **E1/E2/E4** as fillers - each is an afternoon.
6. **D1** only as a deliberately scoped design task.
