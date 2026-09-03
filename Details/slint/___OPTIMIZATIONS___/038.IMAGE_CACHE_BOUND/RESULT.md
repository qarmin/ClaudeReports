# 38 - an image above ~5 MB decoded is never cached, and never even deduplicated

Ported from `perf_test/___OPTIMIZATIONS___/38_image_cache_bound/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

`IMAGE_CACHE` (`core/graphics/image/cache.rs:50-58`) is a `clru` cache bounded at **5 MB of decoded
bytes**, with each entry weighed by its decoded buffer length (`ImageWeightInBytes`). A 1920x1080
RGB8 image decodes to 5.9 MB, more than the whole cache. `clru` rejects an insert that exceeds
capacity, and the call site discards the result:

```rust
self.0.put_with_weight(cache_key, new_image.clone()).ok();   // cache.rs:72
```

## Measured, not argued

`src/bin/image_probe.rs` loads an image, loads it again *while still holding the first*, drops both
and loads once more. A cached image comes back instantly; an uncached one is decoded again.

```
small.png              0.0 MB rgb8   cold    0.09 ms   while held   0.00 ms   after drop    0.00 ms   served from cache
medium.png             0.8 MB rgb8   cold    1.96 ms   while held   0.00 ms   after drop    0.00 ms   served from cache
medium.jpg             0.8 MB rgb8   cold    2.99 ms   while held   0.00 ms   after drop    0.00 ms   served from cache
large.png              5.9 MB rgb8   cold   17.75 ms   while held  14.83 ms   after drop   14.73 ms   RE-DECODED
large.jpg              5.9 MB rgb8   cold   23.65 ms   while held  20.00 ms   after drop   23.85 ms   RE-DECODED
large_alpha.png        5.9 MB rgb8   cold   22.84 ms   while held  22.33 ms   after drop   22.32 ms   RE-DECODED
```

Two things, and the second is the worse one:

1. A 1080p image is **re-decoded on every load** - 14.7 ms for a PNG, 23.9 ms for a JPEG.
2. **It is re-decoded even while the caller still holds the previous `Image`.** The cache is the only
   deduplication there is, so when it refuses the entry, two live references to the same file mean
   two decodes and two copies of the pixels in memory.

The doc comment on the lookup says it will *"upgrade the weak reference to a strong one if found"*.
There are no weak references in the code; the cache holds strong ones and rejects anything large.
Whatever the comment describes, the code does not do it.

## Where the cliff is

The practical question is not "how many bytes" but "how big may my images be":

| asset | decoded (rgb8) | result |
| --- | --- | --- |
| 800x600 | 1.4 MB | cached |
| 1152x864 | 2.8 MB | cached |
| 1360x850 | 3.3 MB | cached |
| 1440x900 | 3.7 MB | cached |
| **1600x1000** | **4.6 MB** | **cached** |
| **1920x1080** | **5.9 MB** | **re-decoded, every time** |

So roughly **1.7 megapixels for RGB8, and 1.3 for anything with an alpha channel**. A 1080p photo -
the most ordinary asset there is - is on the wrong side, and an application gets no warning.

## The prize

Raising the bound to 128 MB, changing nothing else:

| | 5 MB bound | 128 MB bound |
| --- | --- | --- |
| `large.png`, reload | 14.73 ms | **0.00 ms** |
| `large.jpg`, reload | 23.85 ms | **0.01 ms** |
| `large_alpha.png`, reload | 22.32 ms | **0.01 ms** |

The whole decode, every time, is what is being paid.

## Scope - what this does *not* claim

This is a per-**load** cost, not a per-frame one. `Image::load_from_path` runs when a source is set,
so an application whose images are fixed at startup pays it once and never notices. It bites when
sources change: an image list or gallery, thumbnails driven by a model, a viewer stepping through
files, anything that re-assigns `source` as the user scrolls. There it is 15-24 ms per image, on the
UI thread, with no cache to break the cycle - and re-showing an image already on screen costs full
price again.

Not measured yet: whether `@image-url(...)` in `.slint` takes this path at all (it may embed at
compile time), and the same question for the GPU renderers, which upload textures and may keep their
own cache above this one.

## Fix directions

1. **Deduplicate live images regardless of the bound.** A weak-reference side table keyed the same
   way costs no memory - if the application still holds the image, hand back the same one. This fixes
   the worse of the two problems and cannot regress memory, which makes it the one to do first. It is
   also what the existing comment already describes.
2. **Derive the bound from the display**, not a constant: a cache that cannot hold one screenful of
   pixels is not sized for the machine it runs on. 5 MB was presumably chosen for embedded targets,
   where it is right - so this wants to be configurable rather than simply larger.
3. **Stop discarding the insert failure.** `put_with_weight(..).ok()` throws away the one signal that
   would tell a developer their assets are too big to cache. At minimum log it in debug builds.

## Harness

`cargo run --bin make_assets` writes `perf_test/assets/` (36 MB, gitignored): PNG and JPEG at
64x64 / 512x512 / 1920x1080, RGBA variants with a fully opaque and a varying alpha channel, a
six-step series straddling the cache bound, a nine-slice frame and a 400-path SVG. Generated rather
than committed - a benchmark asset nobody can regenerate is one nobody can check.

```sh
cargo run --profile rdebug --bin make_assets
cargo run --profile rdebug --bin image_probe
```

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session. This
candidate quantifies the effect of the existing 5 MB `IMAGE_CACHE` bound (`clru`, rejecting inserts
over capacity) rather than changing it; the 128 MB figure above is a what-if measurement, not an
applied change.
