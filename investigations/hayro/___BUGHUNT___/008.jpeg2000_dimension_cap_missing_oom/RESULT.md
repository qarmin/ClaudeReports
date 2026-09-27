# jpeg2000_dimension_cap_missing_oom

From `___BUGHUNT___/IDEAS.md`'s backlog (noted while root-causing candidate 006): commit
`a7e3aace` ("hayro-jpeg2000: Lift hardcoded restriction on image dimensions", 2026-08-24, two
commits before this session's `HEAD`) **deliberately removed** a pre-existing
`MAX_DIMENSIONS: usize = 60000` per-axis sanity cap from `hayro-jpeg2000/src/j2c/codestream.rs`'s
`size_marker()`, replacing it with `checked_mul`/`checked_add` that only reject true integer
overflow, not "unreasonably large" - the same missing-total-pixel-cap-before-eager-allocation
shape as the already-fixed `hayro-ccitt` (candidate 004) and `DCTDecode` (candidate 006) bugs,
freshly (re)introduced in a different crate (`hayro-jpeg2000`) and left unfixed by the session
that fixed candidate 006.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `hayro-jpeg2000/src/j2c/codestream.rs` (`size_marker`/`size_marker_inner`) and `git show a7e3aace` | Manual code read + `git show` of the removing commit | Confirmed the removed check ran directly inside `size_marker()` on the codestream's raw declared `Xsiz`/`Ysiz` (before any `target_resolution` downscale hint is even computed), and that nothing replaced it anywhere downstream. |
| Traced the actual eager-allocation site: `hayro-jpeg2000/src/j2c/decode.rs::DecoderContext::reset` | Manual code read | Found `sample_count = image_width() * image_height()` (`checked_mul`, overflow-only) drives one `SimdBuffer::zeros(sample_count)` (`vec![0.0_f32; padded_len]`, i.e. a real, eager, non-lazy allocation) per image component, called once per `decode()` **before** any tile-part packet/entropy data is actually decoded (`tile::parse` only locates tile-part boundaries via the `SOT` length field, doesn't decode packets). |
| Hand-crafted a minimal, legitimately-parseable JPEG2000 codestream: took the project's own tiny test fixture `hayro-tests/pdfs/load/different_resolution_levels.jp2` (111 bytes, 2 components, single tile) and hex-patched its SIZ marker's `Xsiz`/`Ysiz`/`XTsiz`/`YTsiz` to `100000` and `XOsiz`/`YOsiz`/`XTOsiz`/`YTOsiz` to `0` (all of `size_marker`'s own tile/offset validation constraints re-checked by hand to still hold), decoded directly via `hayro_jpeg2000::Image::new`/`.decode()` (no PDF involved) | `cargo run --release`, standalone scratch example against the crate's own public API | Header parsed successfully (`width=100000 height=100000`), then `.decode()` aborted: `memory allocation of 80000000000 bytes failed` (Rust allocator failure -> `abort()`) - `100000 * 100000 * 4 bytes/f32 * 2 components`. |
| Wrapped the same patched codestream in a minimal hand-built PDF (`/Filter /JPXDecode` image XObject, classic xref table) and drove it through the real, unmodified `hayro_interpret::interpret_page` + a real `Device` implementation (the exact pattern this project's own shipped `hayro-interpret/examples/extract_images.rs` uses: `RasterImage::with_rgba(_, None)` - no downscale hint) | `cargo run --release` against the project's own tree, 3 separate runs | Every run aborts identically: `memory allocation of 80000000000 bytes failed`, `SIGABRT`, exit code 134 - from a real, adversarial, on-disk-shaped 800-byte PDF, through the real `/JPXDecode` filter dispatch, with no faked participant anywhere in the path. |
| Same PoC PDF driven instead through the top-level `hayro` crate's own `render()` (which passes a CTM-derived `target_dimension` hint, unlike the `interpret_page`/`None` path above) | `cargo run --release`, `timeout 30` | Did **not** abort cleanly within 30s - appears to hang/thrash instead. Traced to `hayro/src/image.rs::draw_pdf_image`'s `target_width` computation multiplying the placement matrix's scale by the image's *raw pixel width* (`transform * Point::new(image.width() as f64, 0.0)`) rather than by a per-pixel-normalized vector, so for this PoC's `cm 100 0 0 100 50 50` placement `target_width` ends up far larger (~10,000,000) than the native 100000px width regardless of how small the image is actually drawn - the resolution-skip optimization doesn't shrink the request here either, it just manifests as a slow hang instead of a clean abort in this particular run. Not chased further as its own candidate this session (see "What's not done"); the `interpret_page`/`Device` path above is the one used for the confirmed, deterministic repro. |

## Status

**Confirmed**. Reproduces 3/3 runs (`--release`, no sanitizer/miri needed - a real allocator
failure, not memory-safety UB) against the pristine `base_commit` tree, and 3/3 runs complete
cleanly (no abort, `interpret_page` returns normally) against the patched tree. See `output.txt`
for the verbatim captured output of both, including the separate `hayro::render` observation.

**Fully reproducible**: **Yes** - `repro/` builds a syntactically real PDF byte-for-byte (classic
xref table, `%PDF-1.7` header, catalog/pages/page/image XObject) with a real, legitimately
SIZ-marker-parseable (only hex-patched dimension fields) JPEG2000 codestream as its `/JPXDecode`
stream, and drives it through the real `hayro_syntax::Pdf::new` + `hayro_interpret::interpret_page`
+ a real `Device` implementation - the exact same public API surface and usage pattern as this
project's own shipped `hayro-interpret/examples/extract_images.rs`. There is no second live
participant to fake here (a malformed file handed to a parser's own public decode entry point is
the genuine external input), same carve-out as candidates 003/004/006 - see `CONFIRMATION.md`
section 11's "single-process repro remains completely fine".

## Class

`security` (resource-exhaustion / memory-safety-adjacent DoS - an attacker-controlled allocation
size, driven entirely by a codestream's own declared `SIZ` marker dimensions and unbounded by the
actual entropy-coded tile data present, reachable from a real PDF via the `JPXDecode` filter).

## Root cause

`hayro-jpeg2000/src/j2c/decode.rs::DecoderContext::reset` computes `sample_count =
header.size_data.image_width() * image_height()` (both derived from the codestream's own `SIZ`
marker `Xsiz`/`Ysiz`, only `checked_mul`-guarded against `usize` overflow, never sanity-capped)
and then allocates one `SimdBuffer::zeros(sample_count)` (a real, eager `vec![0.0_f32;
padded_len]`, 4 bytes/element) per image component - **before** `decode()` reads a single byte of
the actual entropy-coded tile-part data (`tile::parse`, called just before `reset`, only parses
tile-part *boundaries* via the `SOT` marker's own length field, not packet contents). A
pre-existing `MAX_DIMENSIONS: usize = 60000` cap that used to reject this at the `SIZ`-marker
parsing stage (`hayro-jpeg2000/src/j2c/codestream.rs::size_marker`) was deliberately removed in
commit `a7e3aace` ("Lift hardcoded restriction on image dimensions") to let legitimately huge
images be decoded at a requested lower resolution (via `DecodeSettings::target_resolution`), but
nothing replaced it at the point the buffers are actually eagerly allocated - so a request with no
downscale hint (or, per the `hayro::render` observation above, even one nominally carrying a hint
whose own computation doesn't actually shrink anything for this shape of placement) still
allocates the full, attacker-declared `width * height * num_components * 4` bytes up front. This
is the same "declared dimensions drive unbounded eager allocation, independent of the actual
encoded payload" shape as the already-fixed `hayro-ccitt` (candidate 004) and `DCTDecode`
(candidate 006) bugs, in the `hayro-jpeg2000` crate instead.

## Introduced by

a7e3aac "hayro-jpeg2000: Lift hardcoded restriction on image dimensions (#1355)" (2026-08-24) - already identified inline in this candidate's own Root cause section above; confirmed via `git log -1` that this is the exact commit that removed the pre-existing `MAX_DIMENSIONS: usize = 60000` per-axis cap from `size_marker()` without replacing it at the point buffers are actually allocated.

## The fix

Added a `MAX_TOTAL_SAMPLE_COUNT = 64_000_000 * 4` (256 million samples, matching the same order of
magnitude already used for the `hayro-ccitt`/`DCTDecode` guards) ceiling on `sample_count *
num_components`, checked in `DecoderContext::reset` right before the per-component
`SimdBuffer::zeros` allocation loop - i.e. against the **actual** dimensions about to be allocated
(after any `target_resolution`-driven downscaling has already reduced `image_width()`/
`image_height()`), not the codestream's raw declared `Xsiz`/`Ysiz`. This deliberately preserves
`a7e3aace`'s own stated intent (a legitimately huge image decoded at a requested lower resolution
is unaffected by this cap) while closing the gap it left at the point of actual eager allocation.
Exceeding the cap makes `decode()` return `Err(ValidationError::ImageTooLarge)` - an error variant
that already existed and is already handled as an ordinary decode failure by `jpx.rs` (`.ok()?`),
not a panic or abort.

```diff
+const MAX_TOTAL_SAMPLE_COUNT: usize = 64_000_000 * 4;
...
         let sample_count = (header.size_data.image_width() as usize)
             .checked_mul(header.size_data.image_height() as usize)
             .ok_or(ValidationError::ImageTooLarge)?;
+        let total_sample_count = sample_count
+            .checked_mul(initial_tile.component_infos.len())
+            .ok_or(ValidationError::ImageTooLarge)?;
+
+        if total_sample_count > MAX_TOTAL_SAMPLE_COUNT {
+            bail!(ValidationError::ImageTooLarge);
+        }
+
         // TODO: SIMD Buffers should be reused across runs!
         for info in &initial_tile.component_infos {
```

Verified against more than the one triggering input, per `CONFIRMATION.md` section 7: re-ran the
same 800-byte PoC PDF through `repro/` 3 separate times against the unpatched tree (3/3 abort,
`exit 134`) and 3/3 times against the patched tree (3/3 complete cleanly) - see `output.txt`.

- `cargo test -p hayro-jpeg2000 --release`: unit tests **1 passed / 0 failed** before and after
  (identical). The crate's separate `asset_suite` integration test fails identically before and
  after (`Failed to read asset directory: missing test input serenity/buggie-gray.jpf`) - a
  pre-existing environment limitation (downloaded test assets not present in this checkout), not a
  regression; confirmed by running the exact same command against a `git stash`ed unpatched tree.
- `cargo test -p hayro-tests --release --test tests -- load::`: **112 passed / 0 failed**,
  identical before and after - includes every existing JPEG2000 `load::*` fixture (e.g.
  `different_resolution_levels`, `large_tile_offsets`, `progression_iterator_overflow`,
  `lab_conversion_overflow`), confirming the new cap doesn't reject any of the project's own
  existing (legitimately small) JPEG2000 test images.
- `cargo test -p hayro-tests --release` (full suite, all modules): **423 passed / 1284 failed**,
  identical count before and after (confirmed via `git stash` on the fix and re-running the exact
  same command). The 1284 failures are a pre-existing environment limitation (missing downloaded
  reference PDFs/snapshots not present in this checkout, same category noted by candidates
  004/006), not caused by this change.

## What's not done / open questions

- The separate `hayro::render` observation (see "What was tried" and `output.txt`) - the top-level
  renderer's own CTM-derived `target_dimension` hint computation in
  `hayro/src/image.rs::draw_pdf_image` appears to multiply by the image's raw pixel width instead
  of dividing by it, so it doesn't actually shrink the requested resolution for a placement like
  this PoC's, and the same PoC hangs/thrashes for at least 30s through that path instead of
  cleanly aborting - was noticed but not root-caused or fixed as its own candidate this session; it
  may be a related but distinct bug in the renderer's own downscale-hint math, orthogonal to this
  candidate's fix (which the renderer's own hang'll observation doesn't have anything to do with:
  this candidate's fix caps the actual eager allocation, and would also cap the `hayro::render`
  path's own allocation the same way, but doesn't explain or fix the slow-hang behavior itself
  seen within the 30s timeout, which points at extra CPU-bound work elsewhere in the pipeline, not
  the sample-count allocation this candidate targeted. Added to `IDEAS.md`'s backlog as its own
  lead.
- `MAX_TOTAL_SAMPLE_COUNT = 64_000_000 * 4` is a judgment call carried over from candidates
  003/004/006's own precedent, not derived from any JPEG2000/PDF spec limit - a maintainer with a
  different real-world ceiling in mind (e.g. for legitimate multi-band/hyperspectral JPEG2000
  images with many components) should treat this as adjustable, not load-bearing at this exact
  value.
- Did not separately check whether `Csiz` (the component count, up to `65535` per the codestream
  format, capped structurally nowhere in `codestream.rs`) has its own, independent excessive-cost
  issue elsewhere in the parser (e.g. in `read_header`'s `Vec::with_capacity(csiz as usize)` for
  `cod_components`/`qcd_components`, which is bounded but not attacker-cost-free at large `Csiz`
  values) - this candidate's fix does account for `num_components` in the allocation-size check
  itself (`total_sample_count`), so an attacker cannot use a large `Csiz` to bypass the cap, but a
  separate, much-cheaper-than-80GB resource-use path through a large `Csiz` alone was not
  specifically hunted for.
