# Skia: shared image sources re-converted to a texture per item, not per distinct source

`internal/renderers/skia/itemrenderer.rs`'s `draw_image_impl` had a developer-left
`// TODO: avoid doing creating an SkImage multiple times when the same source is used in
multiple image elements`. The per-item `image_cache: ItemCache<Option<skia_safe::Image>>` is
keyed by item identity (component pointer + item index), so N `Image` items that are all bound
to the *same* source `Image` value each independently ran `as_skia_image` (pixel format
conversion, `Data::new_copy`, `skia_safe::images::raster_from_data`) and each got back a
*distinct* `skia_safe::Image` handle - even though the bytes are identical. Skia's own GPU
resource cache keys uploaded textures by an `SkImage`'s internal generation id, so N distinct
handles for the same bytes means N separate GPU texture uploads, repeated on every frame those N
distinct handles are drawn (see "why the result looks the way it does" below - this turned out to
be a bigger, more persistent cost than the TODO's wording suggested).

## What was tried

Added a second cache, `shared_image_texture_cache: RefCell<Vec<(Image, skia_safe::Image)>>`,
alongside the existing per-item `image_cache`. Before converting a source, `draw_image_impl`
checks this cache for an entry whose stored `Image` equals the current one; if found, the already
converted `skia_safe::Image` is reused instead of converting again. `Image`'s own `PartialEq` was
used directly for the identity check ("same source" = same underlying pixel buffer, per
`SharedImageBuffer::eq`, which compares by buffer pointer, not by content) rather than inventing a
second identity notion - see `is_shareable_image_source` in `patch.diff`. Only participates for
sources whose conversion doesn't depend on the requesting item: `EmbeddedImage`, `BackendStorage`
(a previously-cached image), and `NineSlice` wrapping either - explicitly **not** `Svg`, whose
rasterization target size is chosen by whichever item asks first, so sharing it across items
using different sizes would silently return the wrong resolution for one of them. Capped at 128
distinct entries (oldest evicted first), since unlike the per-item cache this one isn't tied to
any item's lifetime and would otherwise never shrink.

An earlier version of this patch keyed the shared cache by `i_slint_core::graphics::ImageCacheKey`
(the identity core's `IMAGE_CACHE` dedupes *decoded* pixels by) instead of by `Image` equality.
That version measured **zero** effect (see "why the result looks the way it does") because
`ImageCacheKey::new` returns `None`/`Invalid` for any image constructed via `Image::from_rgba8`/
`from_rgb8` (by design - "no image cache key can be created ... for programmatically created
images"), which is exactly how `perf_test`'s `images` scene (and plausibly many real apps passing
a `slint::Image` in from Rust rather than an `@image-url` asset) constructs its shared source. The
final version keys on `Image`'s own `PartialEq` instead, which already does the right thing for
both cases (compile-time `@image-url` assets *and* programmatically-shared `Image` values) without
needing to special-case either.

Two `perf_test` harness bugs were fixed along the way (both were pre-existing, reproduced
identically before this patch, and are permanent fixes to `perf_test/` itself - see "Instrumentation
added" in `../README.md`):

1. `--frames --snapshot` panicked with Skia specifically ("RefCell already mutably borrowed"):
   `take_snapshot()` was called from inside the rendering-notifier's own `AfterRendering` callback,
   which re-enters `SkiaRenderer::render()` while its `rendering_notifier` `RefCell` is still
   borrowed for that very callback. Fixed by deferring the snapshot/report/quit to a `Timer`
   scheduled for the next event-loop tick, after the callback's borrow has been released.
2. `--image-distinct`/`--image-size`/`--image-rescale`/`--image-file`/`--image-cell` were parsed
   into `Options` but silently had **no effect** in the windowed (`--frames`) path - `main.rs`'s
   `PerfMode::Images` branch hardcoded `make_image(0, 64)` and never called
   `set_image_distinct`/`set_image_sources`/etc (only the headless `--bench` path, `bench.rs`,
   wired them up). Fixed by mirroring `bench.rs`'s fill logic in `main.rs`.

Both fixes are included as full-file copies in `perf_test_fixes/` (`perf_test/` itself is
untracked in the main repo, so there's no tracked "before" to diff against - see `justfile`).

| Variant | single-core p50 (later frames, `--count 2000 --image-size 64`, shared) | multi-core p50 | notes |
|---|---|---|---|
| baseline | 3.94 ms | 4.13 ms | `perf_test --renderer skia --frames`, `xvfb-run` + `LIBGL_ALWAYS_SOFTWARE=1` |
| patched (`ImageCacheKey`-keyed) | ~3.9 ms (no measurable change) | - | wrong identity notion for this scene, see above - not pursued further, no separate candidate folder |
| patched (`Image`-equality-keyed, final) | 0.86 ms (**-78.2%**) | 0.95 ms (**-77.0%**) | 9/9 paired runs each, `RUNS=9 FRAMES=10` |

First-frame (one-time conversion cost, `--frames 6`, same scene): baseline 31.4 ms -> patched 24.1
ms single-core (**-23.2%**), 32.3 ms -> 24.7 ms multi-core (**-23.5%**), 9/9 paired runs.

At a more modest, realistic on-screen count (`--count 100`, still `--image-size 64`, single-core,
later frames): baseline 1.32 ms -> patched 0.27 ms (**-79.4%**, 9/9 paired) - the win is not an
artifact of the large `--count 2000` used to make it obvious; it shows up at list sizes an actual
app would realistically have on screen.

`--image-distinct` (64 sources cycled over 2000 items, so ~31 items share each texture instead of
2000 sharing 1): baseline 4.03 ms -> patched 3.88 ms (**-3.7%**, single-core, later frames) - real
but much smaller, exactly as expected from the lower sharing factor.

## Result

Real, large, reproducible win for the scenario the TODO names (multiple items sharing one image
source): -78% median frame time at a realistic on-screen count of 100 shared icons, holding at
similar magnitude up to 2000. Confirmed with paired alternating runs (`RUNS=9`, both single-core
`taskset -c 3` and unpinned multi-core), not a single noisy measurement. Only a small win (-3.7%,
close to the noise floor) when items mostly use *distinct* sources, as expected - the fix's value
scales with how much sharing actually exists in a given scene.

## Does it change behavior/output?

- **Bit-exact?** Yes. Checked via `--frames --snapshot` (PPM, MD5) across: shared source (default),
  `--image-distinct` (64 sources), `--image-rescale` (non-1:1 scale), a real PNG loaded via
  `--image-file`, a real SVG loaded via `--image-file shapes.svg` (both at a single size and with
  `--image-rescale`, i.e. two different rasterized sizes of the same SVG in the same run), and the
  large-count (2000) variants of shared/distinct. Every pair (baseline vs patched, same args)
  hashed identical. See "Instrumentation added" in `../README.md` for the swapchain-double-
  buffering race that had to be fixed in the harness first to get a *deterministic* snapshot to
  compare in the first place - without that fix, both baseline and patched independently flip
  between exactly two valid-but-different captured frames depending on timing, unrelated to this
  patch (confirmed: the same two hashes appeared for both binaries, in both configurations, before
  the harness fix).
- **Is this a breaking change?** No - internal renderer implementation detail, no public API
  affected.
- Project test suite: not run (no `i-slint-renderer-skia` unit tests exist to run; the crate has no
  `#[test]`s querying `cargo test -p i-slint-renderer-skia --features high-quality`). Correctness
  relied on the pixel-snapshot comparisons above instead.

## Why the result looks the way it does

The first version of this patch (keyed by `ImageCacheKey`, matching the TODO's own wording -
"core's IMAGE_CACHE dedupes decoded pixel data") measured **exactly zero** difference, allocation
counts included (33509 in both, frame 0, `--count 2000`). Reading `ImageCacheKey::new` explains
why: it returns `None` for anything without a stable path/embedded-resource/SVG identity, which by
its own doc comment is deliberate for "programmatically created images" - exactly what
`Image::from_rgba8` (used by both `perf_test`'s generator and, plausibly, any real app that
decodes/generates an icon in Rust and hands it to Slint as a shared property) produces. The fix
that actually measures a difference uses `Image`'s own `PartialEq` instead, which for
`EmbeddedImage` already compares by the underlying pixel buffer's pointer (see
`SharedImageBuffer::eq` in `internal/core/graphics/image.rs`) - a strictly more general notion of
"same source" than `ImageCacheKey` that covers both the compile-time `@image-url` case (where two
items' sources are literally the same cached buffer, cloned) and the programmatic case.

The magnitude (-78% on *every* frame, not just the first) was larger than a "avoid re-decoding
once" fix should produce, and larger than the -23% first-frame-only win the naive
"N conversions instead of 1" framing predicted. The likely explanation, consistent with the
measured pattern: Skia's `GrResourceCache` keys uploaded GPU textures by an `SkImage`'s
generation id. Before this patch, N items sharing one source held N *distinct* `skia_safe::Image`
handles (one per item, from N separate `raster_from_data` calls) - N separate cache entries
competing for the resource cache's budget, likely evicted and **re-uploaded every frame** rather
than uploaded once and reused, even though the per-item `ItemCache` layer above never re-runs the
CPU-side conversion after frame 0. This patch collapses those N handles to 1, so there is only ever
one GPU resource cache entry to keep resident, explaining why the win persists on every frame
rather than only the first. This is inferred from the measured shape (first-frame *and* steady-
state both improve, by different but both large margins), not confirmed with a GPU-side profiler
(none was available in this environment - see `projects/slint.md`'s `perf` availability note).

## What's not done / open questions

- The `SHARED_IMAGE_TEXTURE_CACHE_LIMIT = 128` bound is a guess, not a measured tuning - unlike
  core's `IMAGE_CACHE` (byte-weighted LRU), this is entry-count-only and FIFO (oldest inserted,
  not oldest used, evicted first). Fine for "a handful of shared icons," not necessarily for an app
  cycling through hundreds of large distinct programmatic images.
- Not wired up for femtovg or the software renderer - unclear whether they have the equivalent
  problem (worth checking; added to `IDEAS.md`).
- No GPU-side profiling confirms the `GrResourceCache` re-upload theory in "why the result looks
  the way it does" - it's the most likely explanation consistent with the measured shape, not a
  directly observed fact.
- `render_into_layer`'s sub-renderer shares the same cache instance (passed through), which is
  correct (same underlying window/context) but wasn't separately stress-tested with layered
  content using a shared image source.
