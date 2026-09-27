# fromimage-oversized-dimensions-silent-truncation

`fromimage/src/main.rs`'s `update_from_img` (shared by both `inject_image` and
`inject_encapsulated`, the two code paths behind the `dicom-fromimage` CLI tool - "replace a DICOM
file's image with another image") stores `PrimitiveValue::from(width as u16)`/`from(height as
u16)` into the Columns/Rows attributes, where `width`/`height` come from the `image` crate's
`img.width()`/`img.height()` (`u32`, unbounded by DICOM's own limits). Found by discovery-sweeping
the previously-unchecked CLI tools (`dump`/`toimage`/`fromimage`/`echoscu`/`findscu`/`movescu`)
queued in `IDEAS.md`'s backlog, using HEURISTICS.md's "arithmetic on sizes/lengths derived from
input" pattern applied to a narrowing cast rather than the read/network side this session mostly
focused on.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: an in-memory `image::DynamicImage` with width `70_000` (just past `u16::MAX`, kept 1-pixel tall to stay cheap to allocate), debug build, `cargo test` (temporary unit test added to `fromimage/src/main.rs`'s existing test module - it already has access to the private `update_from_img` since tests live in the same file/crate; reverted before this record was written) | `update_from_img(&mut obj, &img, false)` called directly (function is private, only reachable from within this same binary crate) | Before any fix: silently stored `Columns = 4464` (`70000 mod 65536`) while the *caller* (`inject_image`, a few lines away, not touched by this specific function) separately writes `img.into_bytes()` - the full, untruncated 70,000-byte real pixel buffer - into `PixelData` right after `update_from_img` returns. The resulting object's Columns attribute describes a completely different image size than what `PixelData` actually holds. Reproduced 3/3 in-tree unit-test runs (1 initial + 2 repeats), then 2/2 more via a standalone `repro/` that replicates the exact vulnerable computation against a fresh clone of pristine upstream `df2b9e42` (5/5 total; see "What's not done" for why `repro/` replicates rather than calls the real function) |
| Second boundary input in the same class (an ordinary, in-range 64x64 image) | Same technique, temporary unit test | Confirmed `update_from_img` still succeeds normally and stores correct Columns/Rows for in-range dimensions, both before and after the fix - the fix doesn't reject valid input |

## Status

**Confirmed** - deterministic, pure-logic bug with no layout/seed/timing dependency; reproduces
5/5 runs across the in-tree unit test (which calls the real, private function directly) and the
standalone `repro/` (which replicates the same computation using only public dependencies, since
`dicom-fromimage` has no library target for `repro/` to call into - see "What's not done"),
identical truncated Columns value every time.

## Class

`logic` (silent data corruption - not a crash. The tool completes "successfully" and writes a
`.dcm` file to disk whose Columns/Rows attributes don't describe the PixelData buffer actually
stored in the same file. No `unsafe`, no panic in the observed manifestation - the corruption is
purely in the written file's own internal consistency, discoverable only by a downstream reader,
possibly much later).

## Root cause

DICOM's Rows/Columns attributes are VR `US` (Unsigned Short), spec-limited to `0`-`65535` - there
is no way to represent an image wider or taller than that in a conformant DICOM file, full stop.
`update_from_img` computes these from the *source* image's real (`u32`, effectively unbounded for
any real-world image format) dimensions via a bare `as u16` cast, which truncates silently instead
of erroring, for the (admittedly narrow, but real) case of a source image exceeding 65535 pixels in
either dimension. Because the *pixel data itself* (`img.into_bytes()`, called separately by each of
`update_from_img`'s two callers) is sized from the same real, untruncated dimensions, the two halves
of the resulting file - metadata and payload - silently disagree once this cast truncates. This is
the same *shape* of bug as the JPEG/JPEG2000 "declared dimensions vs. actual decoded data" mismatch
class fixed in the prior session's candidates `016`/`018`, but on the *encode* side (constructing a
new file from an external image) rather than the *decode* side (reading a codec's output against
DICOM metadata).

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a bounds check at the top of `update_from_img`, before any of its truncating casts, using the
crate's existing `snafu::Whatever`-based error convention (this function's two callers,
`inject_image`/`inject_encapsulated`, already return `Result<()>` and are already propagated via
`?` in `main()` down to a graceful `tracing::error!` + `std::process::exit(-2)` - so `update_from_img`
returning `Result<()>` too, instead of directly calling `std::process::exit` itself, follows the
established pattern rather than introducing a new one):

```
-fn update_from_img(obj: &mut DefaultDicomObject, img: &DynamicImage, verbose: bool) {
+fn update_from_img(obj: &mut DefaultDicomObject, img: &DynamicImage, verbose: bool) -> Result<()> {
     let width = img.width();
     let height = img.height();
     let color = img.color();

+    snafu::ensure_whatever!(
+        width <= u32::from(u16::MAX) && height <= u32::from(u16::MAX),
+        "Image dimensions {width}x{height} exceed the maximum DICOM Rows/Columns can represent ({})",
+        u16::MAX
+    );
+
     let (pi, spp, bits_stored): (&str, u16, u16) = match color { ...
```

(plus the matching `-> Result<()>` return type, `Ok(())` at the end, and `?` added to both call
sites - see `patch.diff` for the full, minimal diff.)

Verified against two inputs in the same class (CONFIRMATION.md section 7): the oversized (70,000
wide) input now returns a descriptive `Err` instead of silently corrupting the output; an ordinary
64x64 image still succeeds normally, confirming the fix doesn't reject valid input.

- Project test suite (`cargo test -p dicom-fromimage`): **2 passed, 1 failed (this candidate's own
  temporary repro test, reverted before writing this record) -> 3 passed, 0 failed** after the fix,
  with both temporary tests included (the oversized-input repro and the valid-dimensions boundary
  check). The pre-existing `verify_cli` test (CLI argument parsing, unrelated to this change) was
  unaffected in both runs.

## What's not done / open questions

- `dicom-fromimage` is a binary-only crate (no `[lib]` target), so - like candidate `042`'s
  `scpproxy` fix - the standalone `repro/` used for this workspace's other candidates (call the
  real, patched library function directly, before/after) doesn't apply cleanly here. `repro/`
  instead replicates the exact vulnerable computation using only public `dicom-core`/`dicom-object`/
  `image` APIs (clearly commented as mirroring the real source lines) - the *actual* patched
  function was verified dynamically during the investigation via a temporary in-tree test (results
  above), not via `repro/`'s own before/after cycle, which only confirms the patched crate still
  builds.
- Whether any code in this workspace (this crate's own pixeldata reader, or an external DICOM
  viewer) would react to a Columns/PixelData mismatch of this shape with a crash rather than a
  graceful error or a silently-wrong-but-non-crashing read was not investigated as part of this
  candidate - `037`/`038` (this session) and `016`/`018` (the prior session) already found and
  fixed several panics in that general shape on other pixel-data code paths, but a mismatched file
  produced specifically by *this* bug was not fed through those paths to check for a chained
  interaction.
- The `image` crate's own documented maximum supported dimensions (if any) for the specific formats
  `dicom-fromimage` decodes (JPEG, PNG, PNM, TIFF, WebP, BMP, EXR) were not researched - it's
  possible some of these formats cap out well under 65,536 pixels per side in practice, which would
  narrow how "realistic" a >65535-pixel *decoded* image actually is for this specific tool, though
  the fix itself is correct regardless (any oversized `DynamicImage`, however it was decoded, would
  hit the same guard).
