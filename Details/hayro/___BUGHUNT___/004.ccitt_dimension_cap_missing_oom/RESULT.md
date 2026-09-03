# ccitt_dimension_cap_missing_oom

`hayro-syntax`'s `CcittFaxDecode` filter (`hayro-syntax/src/filter/ccitt.rs`) has no cap on the
total pixel count it derives from an untrusted PDF image's own `/Width`/`/Height`/DecodeParms
`/Columns`/`/Rows`. From `___BUGHUNT___/IDEAS.md`'s backlog: the same unmerged `Fuzzers` branch
that motivated the now-fixed `hayro-jbig2` candidate (`003.jbig2_unbounded_pixel_decode_hang`)
also applied an identical `width()/height() > 4096` workaround to `fuzz_ccitt.rs`, suggesting the
branch author suspected the same class here too.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `hayro-ccitt` itself for a per-dimension/total-pixel cap (mirroring `hayro-jbig2`'s `MAX_DIMENSION`) | Manual code read of `hayro-ccitt/src/lib.rs` | No cap of any kind inside the crate - but also no eager `width * height`-sized allocation inside it: `push_pixels` clamps each run to the line width and hands it to the caller's `Decoder` trait one run at a time, so the crate's own CPU/memory cost per line is bounded by real decoded runs, not by the declared `columns`/`rows` values themselves. |
| Traced the caller (`hayro-syntax/src/filter/ccitt.rs`, `hayro-interpret/src/x_object/decode/mod.rs`) for where `columns`/`rows` originate and how the decoded buffer is sized | Manual code read | Found the actual defect one layer up: `ccitt.rs::decode` computes `output_len = columns as usize * rows as usize` (only `checked_mul`-guarded against integer *overflow*, not against being absurdly large) from `DecodeParms`' `/Columns`/`/Rows` (maxed against the image XObject's own `/Height`, both fully attacker-controlled, independent of the page's `/MediaBox`), then for any non-indexed color space (`Luma8Decoder`) does `vec![0xFF; output_len]` **unconditionally, before calling `hayro_ccitt::decode` at all** - i.e. before a single byte of the actual encoded stream is read. |
| Manual boundary-value construction: minimal hand-built PDF (766 bytes, classic xref table, ordinary 200x200pt `/MediaBox`) with one `DeviceGray`/1bpc image XObject declaring `/Width 3000000000 /Height 3000000000` and matching `/DecodeParms << /Columns 3000000000 /Rows 3000000000 >>`, rendered through the real, unmodified `hayro::render` public entry point (the same one a PDF viewer built on `hayro` would call) | `cargo run --release` against the project's own tree, `hayro::hayro_syntax::Pdf::new` -> `hayro::render`, 3 separate runs | Every run aborts: `memory allocation of 9000000000000000000 bytes failed` (a Rust allocator failure, `SIGABRT`, exit code 134) - the 4-byte "encoded" stream content is irrelevant, the abort happens purely from the declared dimensions before decoding starts. The page's own render dimensions stay a completely ordinary `200 x 200` the whole time (confirmed by the repro's own printed line) - `hayro-fuzz/fuzz_targets/fuzz_pdf_render.rs`'s existing `MAX_PIX_DIM = 4096.0` guard checks the *page's* render dimensions only, so it would not have caught this class of input either. |

## Status

**Confirmed**. Reproduces 3/3 runs on the unpatched tree (plain `--release` build, no
sanitizer/miri needed - this is an allocator failure from an oversized allocation request, not
memory-safety UB), and 3/3 runs render successfully after the fix. See `output.txt` for the
verbatim captured output of both.

**Fully reproducible**: **Yes** - `repro/` builds a syntactically real PDF file byte-for-byte
(classic xref table, `%PDF-1.7` header, catalog/pages/page/image objects) and drives it through
`hayro::hayro_syntax::Pdf::new` + `hayro::render`, the exact same public entry point a real PDF
viewer built on `hayro` uses for an untrusted file. There is no second live participant to fake
here (a malformed file handed to a parser's own public decode entry point is the genuine external
input, same carve-out as candidate 003 - see `CONFIRMATION.md` section 11's "single-process repro
remains completely fine").

## Class

`security` (resource-exhaustion / memory-safety-adjacent DoS - an attacker-controlled allocation
size, unbounded by the actual encoded stream, reachable from a real PDF via the `CCITTFaxDecode`
filter).

## Root cause

`hayro-syntax/src/filter/ccitt.rs::decode` builds its output buffer eagerly and up front, sized by
values taken directly off the PDF's own (untrusted) dictionaries, with no ceiling other than
64-bit-`usize` overflow protection:

```rust
let columns = params.get::<usize>(COLUMNS).unwrap_or(1728) as u32;
let rows = params.get::<u32>(ROWS).unwrap_or(0).max(image_params.height);
let output_len = (columns as usize).checked_mul(rows as usize)?; // only prevents overflow
...
let mut decoder = Luma8Decoder { output: vec![0xFF; output_len], idx: 0, decoded_rows: 0 }; // eager
```

`columns` comes from `DecodeParms/Columns` (defaults to 1728 if absent) and `rows` from
`max(DecodeParms/Rows, image XObject's own /Height)` - both plain PDF integers with no relation
to the page's `/MediaBox` and no upper bound anywhere in the parsing/interpretation stack (checked
across `hayro-syntax` and `hayro-interpret`: `image_params.width`/`height` are read straight off
the XObject dict via `dict.get::<u32>(WIDTH/HEIGHT)` in `hayro-interpret/src/x_object/image.rs`
with only a `!= 0` check, and `hayro-fuzz/fuzz_targets/fuzz_pdf_render.rs`'s own `MAX_PIX_DIM`
guard checks the unrelated page/viewport dimensions, not any individual image XObject's declared
size). Since `u32::MAX * u32::MAX` (~1.8x10^19) fits well inside `usize::MAX` on a 64-bit target,
`checked_mul` never rejects it - it only guards against wrapping, not against "obviously
un-allocatable". The subsequent `vec![0xFF; output_len]` (in the non-indexed/`Luma8Decoder` path)
then asks the global allocator for that many bytes *before* `hayro_ccitt::decode` is ever called,
so the actual CCITT-encoded stream content (however small) never gets a chance to matter - the
crash happens purely from parsing the dictionaries. This is the same "declared
dimensions/region-size drive unbounded resource use, independent of the actual encoded payload"
shape as the fixed `hayro-jbig2` candidate 003, but manifesting one layer up (an eager allocation
in the PDF-filter glue code) rather than inside the decoder crate itself - `hayro-ccitt`'s own
per-line/per-run decode loop is *not* independently vulnerable to this (see "What was tried"),
since it only ever holds as much state as the actual runs it decodes.

## Introduced by

0ea5dc1 "First stab at ccitt" (2025-05-09) - this commit added `ccitt.rs::decode` (a port of pdf.js's CCITT implementation) with the eager, uncapped `vec![0xFF; settings.columns as usize * image_params.height as usize]` allocation already present. A later commit, 20e49af "hayro-syntax: Allocate correct length for output buffer in CCITT decoder (#1269)" (2026-06-22), added `checked_mul` overflow protection and fixed which value (`rows` vs `image_params.height`) sizes the buffer, but did not add any sanity cap - the unbounded-allocation defect this candidate confirms predates and survives that commit.

## The fix

Added a `MAX_PIXELS = 64_000_000` (64 megapixel) ceiling on `columns * rows` in
`hayro-syntax/src/filter/ccitt.rs`, checked right after `output_len` is computed and before any
allocation or call into `hayro_ccitt::decode` - mirroring the same real-world-generous constant
`candidate 003`'s (unmerged, reverted per this skill's workflow) fix used for `hayro-jbig2`'s
`MAX_PIXELS`. Exceeding the cap makes the filter return `None` (a normal, already-handled decode
failure - the same path taken for any other malformed/unsupported CCITT stream), not a panic or
abort.

```diff
+const MAX_PIXELS: usize = 64_000_000;
...
     let output_len = (columns as usize).checked_mul(rows as usize)?;
+
+    if output_len > MAX_PIXELS {
+        return None;
+    }
```

Verified against more than the one triggering input, per `CONFIRMATION.md` section 7: re-ran the
same 766-byte PoC PDF through `repro/` 3 separate times against the unpatched tree (3/3 abort,
`exit 134`) and 3/3 times against the patched tree (3/3 render successfully, `200x200` pixmap) -
see `output.txt`. Also confirmed via `just test`'s own from-scratch clone-and-patch cycle (see
"What's not done" below on the isolation check).

- Project test suite (`cargo test -p hayro-syntax --release`): **199 passed / 2 failed**,
  identical before and after the patch (confirmed by re-running the exact same command against a
  `git stash`ed unpatched tree). The 2 failures (`pdf::tests::pdf_version_header`,
  `pdf::tests::pdf_version_catalog`) are a pre-existing environment limitation unrelated to this
  change - both panic with `Os { code: 2, kind: NotFound }` trying to read
  `../hayro-tests/downloads/...` assets that are not present in this checkout (not downloaded this
  session), not a real regression.

## What's not done / open questions

- **Isolation check** (`SKILL.md` "Critical pitfalls" #6) was done implicitly rather than by
  reverting only the fixing hunk: the patch is a single 5-line `if` block with nothing else
  changed alongside it, so there is no other candidate change that could be confounding the
  result - re-running the identical `repro/` binary against the unpatched vs. patched tree (both
  captured in `output.txt`) already isolates exactly this one check.
- The indexed-color-space path (`BitPackDecoder`) was not independently exploited here - it grows
  its output `Vec` lazily via `.extend()`/`.push()` as `hayro_ccitt::decode` actually produces
  pixels, so its cost already tracks real decoded work rather than the declared `columns`/`rows`
  up front. It still benefits from the same `MAX_PIXELS` check (applied before either branch), but
  wasn't separately confirmed to be exploitable prior to the fix - noted for completeness, not
  claimed as its own separately-confirmed bug.
- Did not check `hayro-syntax/src/filter/dct.rs`/`jpx.rs` for the same missing-total-pixel-cap
  shape at their own eager-allocation sites (`dct.rs` allocates a decoded JPEG buffer, `jpx.rs`
  sizes buffers from `width`/`height`/`num_components`/`bit_per_component` with only overflow
  `checked_mul` guards, the same pattern as this candidate's root cause) - added to `IDEAS.md`'s
  backlog for a future session rather than checked here, to keep this candidate to one focused
  fix.
- `MAX_PIXELS = 64_000_000` is a judgment call carried over from candidate 003's own reasoning,
  not derived from any CCITT/PDF spec limit (PDF places no upper bound on image `/Width`/`/Height`
  at all) - a maintainer with a different real-world ceiling in mind should treat this as
  adjustable, not load-bearing at this exact value.
