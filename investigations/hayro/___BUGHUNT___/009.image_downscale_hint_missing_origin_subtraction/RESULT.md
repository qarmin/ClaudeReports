# image_downscale_hint_missing_origin_subtraction

From `___BUGHUNT___/IDEAS.md`'s backlog (noted while root-causing candidate 008): a suspicion that
`hayro/src/image.rs::draw_pdf_image`'s CTM-derived `target_width`/`target_height` downscale hint
(passed to `RasterImage::with_rgba`/`StencilImage::with_stencil` so decoders like `hayro-jpeg2000`
can skip decoding resolution levels the page will never show) was computed by *multiplying* the
placement's scale by the image's raw pixel dimensions instead of *dividing* by them, making the
hint far larger than the image's native resolution regardless of how small it's drawn.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `hayro-interpret/src/x_object/image.rs::draw` (the only call site that builds `ImageDrawProps::transform`) and `hayro-interpret/src/types.rs`'s doc comments on `with_rgba`/`with_stencil`'s `target_dimension` parameter | Manual code read | Confirmed the parameter's intended semantics: "a hint for what resolution... you want to have... allows the decoder to extract a lower-resolution version". Also confirmed `transform` passed into `draw_pdf_image` already has `1/width, -1/height` pre-concatenated into the CTM (`context.pre_concat_affine(Affine::new([1.0/width, 0.0, 0.0, -1.0/height, 0.0, 1.0]))`), so plugging the raw pixel width/height back into that same `transform` is mathematically the *correct* way to recover the placement's device-space extent - i.e. the literal "multiply instead of divide" hypothesis from `IDEAS.md` is **not** what's wrong here. |
| Instrumented `draw_pdf_image` with a temporary `eprintln!` printing `transform`/computed `target_width`/`target_height`, rebuilt `hayro`'s own `examples/render.rs`, and ran it (via `cargo build --release -p hayro --example render`) against hand-built real PDFs (classic xref table, real `/JPXDecode` image XObjects) with different `cm` placements of the same image, through the real, unmodified, top-level `hayro::render()` | `cargo run`/direct binary invocation, release build, several hand-built PDFs (100000x100000 malformed codestream from candidate 008's PoC, and a real 1241x1752 multi-resolution-level JPEG2000 fixture) at various `cm` placements | For a placement literally at the coordinate origin with a purely-scaling `cm` (no translation), the computed target *was* correct/small. But for **any** placement with translation - including the coordinate origin's own implicit y-flip translation that the page's initial transform always contributes - the computed target was inflated, sometimes drastically (e.g. a 60x85 on-page thumbnail of a 1241x1752 image placed at `cm 60 0 0 85 500 100` computed a target of `(826, 854)` instead of `(60, 85)`; a `10x10`-on-page placement translated `(1000, 1000)` computed `(1415, 1415)` instead of `(10, 10)`). |
| Root-caused the mechanism: `(transform * Point::new(image.width(), 0.0)).to_vec2().length()` never subtracts `transform * Point::new(0.0, 0.0)`. Verified the fix (subtracting the transformed origin before taking the vector length) with the same instrumented build | Same instrumented release build, same PDFs, before/after a one-line-per-dimension source edit | With the fix, every placement tested (near-origin, far-offset, the realistic thumbnail) computed the correct, placement-independent target (`(60, 85)` for the thumbnail regardless of `cm` translation). |
| Built a real, standalone, end-to-end repro (`repro/`) using only the real public `hayro::render()` API (no source instrumentation) on a real, non-malformed, multi-resolution-level JPEG2000 fixture, and measured wall-clock render time (median of 5 runs) for two ordinary placements of a 60x85 thumbnail on a normal 612x792 page | `cargo run --release`, 3 separate full runs against the pristine `base_commit` tree (via `just test`'s `clone` + unpatched build) and 3 separate runs against the patched tree | Reproduces 3/3: unpatched median times ~16-34ms for **both** placements (the bug affects even the nominally-"near-origin" placement, because of the page's own inherent y-flip translation); patched median times ~0.7-1.8ms for both - an approximately 15-40x speedup from a one-line-per-dimension fix, with the ratio consistent across repeated runs. |

## Status

**Confirmed**. The literal formula hypothesized in `IDEAS.md` ("multiply instead of divide by the
image's raw pixel width/height") is refuted by direct execution - that part of the computation is
correct. A different, real bug lives in the same two lines: the transformed image origin is never
subtracted before taking the vector length, so the placement's own device-space translation (which
is essentially always non-zero for a real PDF, since the page's initial y-flip transform alone
contributes one) leaks into what's supposed to be a pure on-page-extent measurement, systematically
inflating the downscale hint. Reproduces 3/3 runs (release build, via `just test` against a fresh
clone of the pristine `base_commit` tree) before the fix, and 3/3 runs after.

**Fully reproducible**: **Yes** - `repro/` drives the real, public, top-level `hayro::render()`
entry point (the exact function `hayro/examples/render.rs` itself calls) on a real, legitimately
fully-decodable JPEG2000 image (not header-patched) embedded in a real, on-disk-shaped PDF with a
real `/JPXDecode` image XObject and a real `cm` placement - the same ordinary "photo/thumbnail
somewhere on a normal page" layout any real-world PDF uses. There is no second participant to fake
here (a PDF file handed to the real public renderer is the genuine external input), same carve-out
as candidates 003/004/006/008 - see `CONFIRMATION.md` section 11's "single-process repro remains
completely fine".

## Class

`logic` (a silent performance-defeating computation bug, not a crash/UB - the resolution-skip
optimization this hint exists for stays effectively disabled for ordinary placements, so
`hayro-jpeg2000`, and any other downscale-hint-aware decoder, decodes many more resolution levels
of wavelet/DCT/etc. data than the page will ever display).

## Root cause

`hayro/src/image.rs::draw_pdf_image` computes:

```rust
let target_width = (transform * Point::new(image.width() as f64, 0.0))
    .to_vec2()
    .length()
    .ceil() as u32;
```

`transform` already has the image's own `1/width, -1/height` pixel-to-unit-square normalization
pre-concatenated into the placement's CTM (done once, in
`hayro-interpret/src/x_object/image.rs::draw`), so `transform` maps *raw pixel coordinates*
directly to *device-space coordinates* - multiplying by `image.width()` here is the correct way to
ask "where does the far edge of the image land in device space." The bug is that `Affine * Point`
in `kurbo` includes the affine's translation component, and `.to_vec2()` just reinterprets that
resulting *absolute device-space point* as a vector measured from the coordinate system's own
origin `(0, 0)` - not as a vector relative to where the image's own near edge (`transform *
Point::new(0.0, 0.0)`) landed. Whenever the placement's transform carries any translation (i.e.
essentially always - a plain axis-aligned `cm` translation, or simply the page's own initial
y-flip transform, which always contributes a translation equal to roughly the page height), that
translation is added into what should be a pure extent measurement, inflating `target_width`/
`target_height` by an amount proportional to how far from device `(0, 0)` the placement happens to
land. The same bug affects `target_height`'s analogous computation.

## Introduced by

a5083ef "Add a way of specifying a size hint when decoding images (#641)" (2025-12-09) - `git blame` on the current `target_width` computation in `hayro/src/image.rs` (originally `hayro/src/renderer.rs` before a later module split) attributes it to this commit, which introduced the `transform * Point::new(image.width(), 0.0)` calculation with the origin-subtraction bug already present.

## The fix

Subtract the transformed origin from both transformed corner points before taking the vector
length, so only the linear (scale/shear/rotation) part of the transform contributes to the
computed extent, not its translation:

```diff
-        let target_width = (transform * Point::new(image.width() as f64, 0.0))
-            .to_vec2()
+        let origin = transform * Point::new(0.0, 0.0);
+        let target_width = (transform * Point::new(image.width() as f64, 0.0) - origin)
             .length()
             .ceil() as u32;
-        let target_height = (transform * Point::new(0.0, image.height() as f64))
-            .to_vec2()
+        let target_height = (transform * Point::new(0.0, image.height() as f64) - origin)
             .length()
             .ceil() as u32;
```

Verified against more than the one triggering input, per `CONFIRMATION.md` section 7: re-ran the
same standalone `repro/` (both an "ordinary thumbnail" placement and a "translate `(0,0)`"
placement) 3 separate times against the unpatched tree and 3 separate times against the patched
tree - every unpatched run computed an inflated, placement-dependent target and took tens of
milliseconds for both placements; every patched run computed the correct, placement-independent
target (`60x85` for this repro's thumbnail) and took under ~2ms for both placements. Also manually
re-checked two further placements outside the repro's own two (from the "What was tried" table: a
`10x10`-on-page image translated far from the origin, and the original far-offset PoC from
candidate 008) - the fix produced the correct, translation-independent target in every case tried.

- Project test suite:
  - `cargo test -p hayro --release`: 0 tests in this crate (`N/A`), identical before and after.
  - `cargo test -p hayro-tests --release --test tests -- load::`: **112 passed / 0 failed**,
    identical before and after.
  - `cargo test -p hayro-tests --release --test tests -- render::`: **266 passed / 1259 failed**,
    **identical count and identical failing-test set before and after** (confirmed via `git stash`
    on the fix and re-running the exact same command against the pristine tree) - the 1259 failures
    are the same pre-existing environment limitation candidates 004/006/008 already documented
    (missing downloaded reference PDFs/snapshots not present in this checkout), not caused by this
    change. In particular, the three JPEG2000-specific render tests that do fail with a real "pixel
    diff" rather than a missing-file error (`stream_jpx_3`/`stream_jpx_5`/`stream_jpx_6`) fail with
    the *same* pixel-diff magnitudes on the pristine tree as on the patched one (e.g.
    `stream_jpx_5`: `91173` both before and after) - this fix does not change their outcome or
    their visual output in any observable way for these fixtures.

## What's not done / open questions

- The original observation that motivated this candidate (candidate 008's note: the same
  100000x100000 malformed-codestream PoC PDF hangs/thrashes for 30+ seconds through
  `hayro::render()` instead of cleanly aborting like it does through `interpret_page` + a `Device`
  with no hint) is **not** explained or fixed by this candidate. Re-checked directly: even with
  this fix applied, driving that exact PoC through `hayro::render()` still hangs past a 20s
  timeout, because that PoC's underlying JPEG2000 fixture only has a small number of real
  resolution levels (`min_num_resolution_levels` in
  `hayro-jpeg2000/src/j2c/codestream.rs::size_marker`), so even a now-correctly-computed small
  target hint can only trigger `skipped_resolution_levels` up to that small cap - nowhere near
  enough to shrink a header-patched `100000x100000` declared size down to something safe. That
  hang is a distinct, not-yet-root-caused issue (added back to `IDEAS.md` as its own lead) -
  separately, candidate 008's own fix (a `MAX_TOTAL_SAMPLE_COUNT` cap in
  `DecoderContext::reset`) already guards the *allocation* aspect of that same PoC regardless of
  this candidate's fix; whether the hang itself is CPU-bound work elsewhere in the decode pipeline
  that similarly needs a cap was not investigated this session.
- Did not check whether the same "transformed point used as an absolute vector" pattern
  (`.to_vec2()` on a `Point` produced by an `Affine` that carries translation, without subtracting
  a reference point first) appears anywhere else in `hayro`/`hayro-interpret` - this session only
  looked at this one call site.
- The practical impact scales with how far from device `(0, 0)` a given placement's own origin
  lands and with the image's own real number of available resolution levels; this session's
  `repro/` demonstrates a representative ~15-40x slowdown for one ordinary thumbnail placement of
  one real multi-level image on one dev machine - the exact ratio is not a portable/precise number
  and will vary by placement, image, and machine.
