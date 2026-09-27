# dct_dimension_cap_missing_oom

`hayro-syntax`'s `DCTDecode` filter (`hayro-syntax/src/filter/dct.rs`) caps an untrusted PDF
image's own `/Width`/`/Height` at `u16::MAX` each (so it cannot repeat candidate 004's
astronomical-dimension trick on width/height alone), but never caps the *decoded output buffer*
size (`width * height * num_components`) before eagerly allocating it via `zune_jpeg`'s
`decoder.decode()`. From `___BUGHUNT___/IDEAS.md`'s backlog (noted while root-causing candidate
004): the same missing-total-pixel-cap-before-eager-allocation shape, one layer up from
`hayro-ccitt`/`hayro-jpeg2000`, applied to the JPEG (DCT) filter.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `hayro-syntax/src/filter/dct.rs` for the allocation site and its existing guards | Manual code read | Found the existing `image_params.width/height > u16::MAX` early-return, but no cap anywhere on the multiplication by `num_components` used for the *output* colorspace, nor a total-byte-count sanity check before `decoder.decode()`. |
| Read `zune-jpeg` 0.5.15's own `JpegDecoder::decode()` | Manual code read of the vendored crate (`~/.cargo/registry/.../zune-jpeg-0.5.15/src/decoder.rs`) | Confirmed `decode()` calls `decode_headers()` (parses only markers up to SOS, no scan/entropy data needed), then unconditionally does `let size = self.output_buffer_size().unwrap(); let mut out = vec![0; size];` **before** `decode_into()` ever touches the actual entropy-coded scan bytes - i.e. the allocation size is driven entirely by declared header metadata (width/height/output colorspace component count), not by how much real encoded data is present. `output_buffer_size()` only overflow-checks via `checked_mul`, never sanity-caps the result. |
| Isolated `zune-jpeg` test: a real 1x1 JPEG (from ImageMagick) with its SOF0 `width`/`height` fields hex-patched to `0xFFFF`/`0xFFFF`, decoded directly (no `hayro` involved) with `DecoderOptions::default().set_max_width/height(u16::MAX)` | `cargo run --release`, standalone `zj` scratch crate | `decode()` succeeded and returned a `Vec<u8>` of `12,884,508,675` bytes (65535 * 65535 * 3, RGB) - confirmed via `/proc/self/status` that this was a *real* resident allocation (`VmRSS` went from ~2.5 MB to ~12.58 GB), not a lazily-uncommitted virtual reservation, taking several real seconds. This system (29 GB RAM + 8 GB swap) happened to have enough memory to satisfy it, but a memory-limited host/container would not. |
| Manual boundary-value construction: minimal hand-built PDF (1261 bytes, classic xref table, ordinary 200x200pt `/MediaBox`) with one `DCTDecode` image XObject declaring `/Width 65535 /Height 65535` (the JPEG's own SOF0 hex-patched to match) and a `/DeviceN` colorspace with 20 colorant names (pushing `dct.rs`'s `ColorSpace::MultiBand(20)` branch, so the eventual allocation is `65535 * 65535 * 20`  bytes instead of only 3/4), rendered through the real, unmodified `hayro::render` public entry point | `cargo run --release` against the project's own tree, `hayro::hayro_syntax::Pdf::new` -> `hayro::render`, 3 separate runs | Every run aborts: `memory allocation of 85896724500 bytes failed` (a Rust allocator failure, `SIGABRT`, exit code 134) - unconditionally, on this same 29 GB/8 GB-swap host, no `ulimit` needed. The 286-byte embedded JPEG's actual scan data is irrelevant (it's the unmodified entropy-coded data for a real 1x1 image); the abort happens purely from parsing the image dictionary + JPEG header. The page's own render dimensions stay an ordinary `200 x 200` throughout (confirmed by the repro's own printed line). |

## Status

**Confirmed**. Reproduces 3/3 runs on the unpatched tree (plain `--release` build, no
sanitizer/miri needed - this is an allocator failure from an oversized allocation request, not
memory-safety UB), and 3/3 runs render successfully after the fix. See `output.txt` for the
verbatim captured output of both.

**Fully reproducible**: **Yes** - `repro/` builds a syntactically real PDF file byte-for-byte
(classic xref table, `%PDF-1.7` header, catalog/pages/page/image/function objects) and drives it
through `hayro::hayro_syntax::Pdf::new` + `hayro::render`, the exact same public entry point a
real PDF viewer built on `hayro` uses for an untrusted file. There is no second live participant
to fake here (a malformed file handed to a parser's own public decode entry point is the genuine
external input, same carve-out as candidates 003/004 - see `CONFIRMATION.md` section 11's
"single-process repro remains completely fine").

## Class

`security` (resource-exhaustion / memory-safety-adjacent DoS - an attacker-controlled allocation
size, unbounded by the actual encoded stream, reachable from a real PDF via the `DCTDecode`
filter).

## Root cause

`hayro-syntax/src/filter/dct.rs::decode` determines an output colorspace/component count for the
decoded JPEG - either from the JPEG's own header (up to 4 components: CMYK/YCCK) or, if the PDF
image's `/ColorSpace` is a `/DeviceN` array, from that array's own colorant-name count cast to
`u8` (up to 255, taken via `ColorSpace::MultiBand(NonZeroU32::new(num_components as u32)?)`) -
then calls `zune_jpeg`'s `decoder.decode()`, which internally does
`vec![0; width * height * num_components]` **before** decoding a single byte of the actual
entropy-coded scan data. `width`/`height` here are bounded to `u16::MAX` (65535) each by an
existing check earlier in the function, but nothing bounds `width * height * num_components`
itself: the existing `u16::MAX` guard only stops candidate 004's astronomical-dimension trick, not
this one, since 65535 * 65535 alone (~4.3 billion) times even the ordinary 3/4-component case
(~12.9-17.2 GB) is already a large, real, resource-exhaustion-class allocation, and the
`/DeviceN`-driven component count multiplies that further by up to another 255x with no cap
anywhere in the path (checked across `hayro-syntax`'s `dct.rs`, `zune-jpeg`'s own
`output_buffer_size()`, and `hayro-interpret`'s `DeviceN::new()`, which only checks
`u8::try_from(names.len())` for *overflow*, not for a real-world-sane colorant count). This is the
same "declared dimensions/metadata drive unbounded resource use, independent of the actual encoded
payload" shape as the fixed `hayro-ccitt` candidate 004, manifesting in the JPEG/DCT filter glue
code instead of the CCITT one - and, unlike candidate 004's `columns`/`rows` (which come straight
from `/DecodeParms`, unbounded), `dct.rs` already had a *partial* guard (`u16::MAX` on width/height
alone) that turned out to be insufficient once the `/DeviceN` component-count multiplier is
accounted for.

## Introduced by

72d43d8 "Bump zune_jpeg and support multi-band images (#358)" (2025-10-08) - this commit introduced the `/DeviceN`-array-driven `ColorSpace::MultiBand(NonZeroU32::new(num_components as u32)?)` path (the up-to-255x component-count multiplier the root cause describes), with no cap on the resulting `width * height * num_components` allocation size.

## The fix

Added a `MAX_OUTPUT_BYTES = 64_000_000 * 4` (256 million byte, i.e. a 64-megapixel image at up to
4 components) ceiling on `width * height * num_components`, computed from the JPEG decoder's own
already-parsed dimensions and the already-determined output colorspace, checked right before
`decoder.set_options(...)`/`decoder.decode()` - mirroring the same real-world-generous constant
candidate 004's `MAX_PIXELS` fix used for `hayro-ccitt`. Exceeding the cap makes the filter return
`None` (a normal, already-handled decode failure - the same path taken for any other
malformed/unsupported JPEG stream), not a panic or abort.

```diff
+const MAX_OUTPUT_BYTES: usize = 64_000_000 * 4;
...
+    let (dec_width, dec_height) = decoder.dimensions().unwrap();
+    let output_len = dec_width
+        .checked_mul(dec_height)?
+        .checked_mul(out_colorspace.num_components())?;
+
+    if output_len > MAX_OUTPUT_BYTES {
+        return None;
+    }
+
     decoder.set_options(DecoderOptions::default().jpeg_set_out_colorspace(out_colorspace));
     let mut decoded = decoder.decode().ok()?;
```

Verified against more than the one triggering input, per `CONFIRMATION.md` section 7: re-ran the
same 1261-byte PoC PDF through `repro/` 3 separate times against the unpatched tree (3/3 abort,
`exit 134`) and 3/3 times against the patched tree (3/3 render successfully, `200x200` pixmap) -
see `output.txt`. Also separately confirmed the plain-3-component (`/DeviceRGB`, no `/DeviceN`)
variant still requests a real (not just virtual) ~12.88 GB allocation before the fix (measured via
`/proc/self/status` `VmRSS`), which the fix also caps.

- Project test suite (`cargo test -p hayro-syntax --release`): **199 passed / 2 failed**,
  identical before and after the patch (confirmed by re-running the exact same command against a
  `git stash`ed unpatched tree). The 2 failures (`pdf::tests::pdf_version_header`,
  `pdf::tests::pdf_version_catalog`) are a pre-existing environment limitation unrelated to this
  change - both panic with `Os { code: 2, kind: NotFound }` trying to read
  `../hayro-tests/downloads/...` assets that are not present in this checkout (not downloaded this
  session), not a real regression - identical to candidate 004's own observation of the same two
  failures.

## What's not done / open questions

- **`jpx.rs`** (`hayro-syntax/src/filter/jpx.rs`, the `JPXDecode`/JPEG2000 filter) was the other
  half of this candidate's original suspicion and was *not* fixed here - it turned out to be a
  substantially bigger and more directly-confirmed lead than expected while root-causing this one:
  a very recent upstream commit, `a7e3aace` ("hayro-jpeg2000: Lift hardcoded restriction on image
  dimensions", 2026-08-24, i.e. two commits before this session's `HEAD`), **deliberately removed**
  a pre-existing `MAX_DIMENSIONS: usize = 60000` sanity cap in
  `hayro-jpeg2000/src/j2c/codestream.rs`'s `size_marker()`, replacing it with `checked_mul`/
  `checked_add` that only reject true `usize`/`u32` overflow, not "unreasonably large" - the exact
  same bug shape as this candidate and as candidate 004, but freshly (re)introduced in a different
  crate (`hayro-jpeg2000`, consumed by `jpx.rs`) rather than checked/fixed here. This is a
  materially different root cause/fix location (a different crate entirely) from this candidate's
  `dct.rs` fix, so per this skill's "one bug per candidate folder" rule it was **not** bundled into
  this same patch/repro - it has been written back into `IDEAS.md`'s backlog as its own, much
  sharper lead (citing `a7e3aace` directly) for a following candidate, rather than investigated to
  a full reproduction in this session's time.
- `MAX_OUTPUT_BYTES = 64_000_000 * 4` is a judgment call carried over from candidate 003/004's own
  reasoning (a "64-megapixel-at-4-components" ceiling), not derived from any JPEG/PDF spec limit -
  a maintainer with a different real-world ceiling in mind should treat this as adjustable, not
  load-bearing at this exact value.
- Did not separately re-verify whether `maybe_patch_jpeg_dimensions`'s own patching logic (which
  shrinks an embedded JPEG's *own* SOF dimensions down to the PDF image dictionary's declared
  `/Width`/`/Height` when the JPEG's own metadata claims to be larger) has any bearing on this bug
  - it doesn't: that function only ever *shrinks* the JPEG's declared size to match
  `image_params`, so it cannot itself introduce a larger allocation than what `image_params`
  (already capped at `u16::MAX` per axis) or the JPEG's own unmodified SOF metadata would produce;
  this candidate's fix caps the actual post-`decode_headers()` dimensions regardless of which of
  the two paths produced them.
