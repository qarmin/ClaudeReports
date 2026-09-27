# 62 - premultiply once on `Image::from_rgba8`, not per pixel per frame

`internal/renderers/software/draw_functions.rs`'s `fetch_blend_pixel` has a `TexturePixelFormat::Rgba`
arm that calls `PremultipliedRgbaColor::premultiply()` (3 multiplies + 3 divides-by-255) for every
destination pixel drawn, every frame, with no shortcut for the common opaque case. A
`TexturePixelFormat::RgbaPremultiplied` arm already exists in the same match and, when there is no
tint color, its `alpha == 0xff` branch skips that arithmetic entirely (direct byte copy into the
blend). File-decoded images (`Image::load_from_path`) already reach the premultiplied format - they
are converted once at decode by `dynamic_image_to_shared_image_buffer`. `Image::from_rgba8`, the
public constructor for a straight-alpha buffer supplied directly by the app, did not: it stored
`SharedImageBuffer::RGBA8` (straight alpha) verbatim, so every one of its images paid the
premultiply arithmetic on every pixel of every frame for as long as the (otherwise immutable) image
was drawn. `perf_test`'s own `images` scene already uses this exact constructor
(`generator.rs::make_image`, `slint::Image::from_rgba8(buffer)`) for its default and
`--image-distinct` sources, with alpha = 255 everywhere - the textbook "recomputes the same answer
every frame" case this candidate targets.

## What was tried

One variant: `Image::from_rgba8` now premultiplies the buffer once at construction (reusing the
existing `Image::rgba_to_premultiplied_rgba` helper, the same one `dynamic_image_to_shared_image_buffer`
already uses for decoded files) and stores `SharedImageBuffer::RGBA8Premultiplied` instead of
`SharedImageBuffer::RGBA8`. No renderer code was touched - GPU renderers (femtovg, skia) already
convert either variant to a texture once per source and cache it (candidate 056), so they were not
expected to move and weren't specifically re-measured; only the software renderer's per-frame
`fetch_blend_pixel` path is affected.

`perf_test --bench --mode images` (software renderer, `--repaint-buffer new` to force a full
repaint every frame so the draw path is actually exercised - without it this static scene has zero
dirty region after frame 0), `RUNS=9`, `FRAMES=20`, later-frames median via `tools/paired.sh`:

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 | notes |
|---|---|---|---|
| baseline, `--count 200 --image-cell 200` | 4.46 ms | 4.47 ms | |
| patched, `--count 200 --image-cell 200` | 2.18 ms (**-51.1%**, 9/9) | 2.17 ms (**-51.5%**, 9/9) | |
| baseline, `--count 64` (defaults: 64px images, 68px cell) | 1.58 ms | - | |
| patched, `--count 64` | 0.93 ms (**-41.1%**, 9/9) | - | |
| baseline, `--count 2000 --image-cell 200 --image-distinct` | 4.87 ms | - | 64 distinct sources |
| patched, `--count 2000 --image-cell 200 --image-distinct` | 2.59 ms (**-46.8%**, 9/9) | - | |

Single-core and multi-core agree (this is a single-threaded software-rasterizer path; the
`--count 200 --image-cell 200` case was the only one run both ways, since the mechanism gives no
reason to expect a difference and none was found).

## Result

Real, large, and consistent: -41% to -51% on later-frame time across three parameter combinations
(default size, larger drawn cells, and 64 distinct sources), single- and multi-core, every
comparison 9/9 paired wins. This is a much larger effect than candidate 039's earlier estimate
("worth at most 0.10 ms of a 0.27 ms gap") - that estimate was for **file-decoded** RGBA images,
which (both then and now) are already converted to `RgbaPremultiplied` at decode and mostly measure
a different cost (the mandatory `.blend()` call, not the premultiply arithmetic - see "Why" below).
This candidate's win comes specifically from `Image::from_rgba8`-constructed images, which is a
different, previously-unmeasured code path.

## Does it change behavior/output?

- **Bit-exact for the harness's own images?** Yes - `--mode images` default and `--image-distinct`
  sources use alpha = 255 everywhere (`generator.rs::make_image`), and `rgba_to_premultiplied_rgba`
  is an exact no-op at alpha = 255 (`if pixel.a == 255 { pixel }`). PPM snapshots (`--snapshot`,
  headless) are byte-identical baseline vs. patched for both the default-source and
  `--image-distinct` scenes (`cmp` reports no difference).
- **Bit-exact in general? No, for translucent pixels.** `Image::from_rgba8(...).to_rgba8()` is no
  longer guaranteed to return the exact input bytes for alpha strictly between 0 and 255 - the
  round trip now goes through premultiply-then-unpremultiply, which is lossy (integer rounding both
  ways). Measured directly (added-then-reverted unit test sweeping all 255x254 color/alpha
  combinations other than alpha=0/255, which are already covered and exact by the project's
  existing `test_rgb_to_premultiplied_*`/`test_premultiplied_to_rgb_*` unit tests in
  `internal/core/graphics/image.rs`):

  | alpha | 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128 | 192 | 255 |
  |---|---|---|---|---|---|---|---|---|---|---|
  | max per-channel diff | 127 | 64 | 32 | 16 | 8 | 4 | 2 | 1 | 1 | 0 |

  The error is inversely proportional to alpha (roughly `127/alpha`), i.e. worst for
  nearly-invisible pixels (alpha=1, where the color barely contributes to the final blended
  result) and negligible (<=1 LSB) from alpha=128 up; alpha=0 and alpha=255 remain exact (both
  already covered by pre-existing project unit tests). A caller relying on an exact
  `from_rgba8`/`to_rgba8` round trip for translucent pixels (e.g. an image-editing round-trip
  test) would notice this.
- **Node.js binding interaction (found while investigating this, not fixed here):** the Node.js
  `SlintImageData.data` getter (`api/node/rust/types/image_data.rs`) has a pre-existing bug in its
  `RGBA8Premultiplied` arm (wrong byte stride, discards alpha) that is unrelated to this candidate,
  but this patch would widen which images hit it (`new ImageData(w, h)` / any
  `Image::from_rgba8`-constructed image, previously served correctly by the `RGBA8` arm). See
  `___OTHER___/002.NODE_IMAGEDATA_PREMULTIPLIED_BYTE_MISMATCH/` for the full write-up and a
  standalone repro - not duplicated here.
- **Is this a breaking change?** Only in the narrow sense above (round-trip precision for
  translucent `from_rgba8` images) and its interaction with the pre-existing Node.js bug. The
  `Image::from_rgba8` doc comment does not promise exact round-tripping today, and
  `from_rgba8_premultiplied` already exists as the "I already have premultiplied data" escape
  hatch for callers who need to avoid this.
- **Project test suite:** `i-slint-core --lib` 307/307 passed, unchanged, with and without
  `--features testing` (both before and after the patch).

## Why the result looks the way it does

`fetch_blend_pixel`'s `Rgba` arm always executes `PremultipliedRgbaColor::premultiply()` (3x
`u16` multiply + divide-by-255), regardless of whether the per-pixel alpha is 255. The
`RgbaPremultiplied` arm's `alpha == 0xff` (no tint) branch does none of that - it constructs the
blended color directly from the stored bytes. Moving `Image::from_rgba8`'s images into the
premultiplied representation once, at construction, means every subsequent frame's draw calls take
that already-fast branch instead. Candidate 039's smaller estimate for "premultiply once" was about
a different, already-optimized code path (file decode); the arithmetic candidate 039 measured as
"only a third of the gap" for an RGBA8-opaque **file** was actually mostly the unavoidable
`.blend()` call rather than the premultiply itself (that arm was already `RgbaPremultiplied`, not
`Rgba`, at the time - see that candidate's "Refuted" list). This candidate's win is real and
distinct: it's the `Rgba` arm's own arithmetic, on a code path (`from_rgba8`) that had never been
measured before.

## What's not done / open questions

- Not extended to the C++ binding: `api/cpp/include/private/slint_image.h`'s `Image(SharedPixelBuffer<Rgba8Pixel>)`
  constructor builds `SharedImageBuffer::RGBA8` directly in the generated C++ header, bypassing the
  Rust `Image::from_rgba8` function this patch touches entirely. C++ callers of the equivalent
  constructor would not get this improvement without a separate, C++-side change.
- The pre-existing Node.js `SlintImageData.data` bug this investigation surfaced is filed
  separately in `___OTHER___/002.NODE_IMAGEDATA_PREMULTIPLIED_BYTE_MISMATCH/` and not fixed here.
- Not measured on GPU renderers (femtovg/skia) - by code reading they already upload/cache a
  texture once per source regardless of which `SharedImageBuffer` variant it is
  (`image_buffer_to_skia_image`, `image_buffer_to_image_source`), so no per-frame effect is
  expected, but this wasn't independently confirmed with a `--frames`/Xvfb run in this session.
- The construction-time premultiply itself is a new allocation + O(pixel count) pass at
  `from_rgba8` call time (previously the buffer was reused verbatim). This is a net win for an
  image drawn many times without being reconstructed (this candidate's whole premise), but would be
  a wash or a slight loss for a caller that reconstructs a new `Image::from_rgba8` every single
  frame (e.g. streaming video into a fresh buffer each frame) and only ever draws it once - not
  measured here, flagged as a caveat.

## Reproduce

```sh
cd internal/core && cargo test --lib   # 307 passed, unchanged

cd perf_test
cargo build --profile rdebug --bin perf_test --features software
../target/rdebug/perf_test --bench --mode images --count 200 --image-cell 200 \
    --repaint-buffer new --frames 20

# paired before/after (from the main ___OPTIMIZATIONS___/ directory, two binaries built from
# git-stash-applied/reverted internal/core/graphics/image.rs):
RUNS=9 FRAMES=20 taskset -c 3 tools/paired.sh <base-bin> <patched-bin> -- \
    --mode images --count 200 --image-cell 200 --repaint-buffer new

# correctness (pixel-identical for the harness's own opaque images):
./<base-bin> --bench --mode images --count 200 --image-cell 200 --frames 5 --snapshot /tmp/a.ppm
./<patched-bin> --bench --mode images --count 200 --image-cell 200 --frames 5 --snapshot /tmp/b.ppm
cmp /tmp/a.ppm /tmp/b.ppm   # no output = identical
```
