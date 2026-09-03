# TIFF_CMYK_SCRATCH_BUFFER_DOUBLE_ACCOUNTING

Follow-up to candidate 038's own noticed-but-unswept backlog item: does any decoder's `read_image`
allocate a decoder-internal scratch buffer *larger* than `layout.total_bytes()` (unlike OpenEXR's
equal-size case), which would slip past `ImageReader::decode_to_dynimage`'s centralized
`self.limits.reserve(layout.total_bytes())` call? `src/codecs/tiff.rs` is the one decoder in this
crate whose `read_image` genuinely does this: for a CMYK8/CMYK16 source, its internal decode
buffer is sized for the *original* 4-channel CMYK data (via `image-tiff`'s own
`read_image_to_buffer`) and only converted down to the smaller 3-channel RGB output afterwards -
the code even has a pre-existing comment acknowledging this ("The buffer can be larger for CMYK
than the RGB output").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: every `read_image` implementation in `src/codecs/` for an internal `vec![...]`/decode-buffer allocation whose size could exceed `layout.total_bytes()` | Static read of current `main` (`44ce9226`) | Found `tiff.rs`'s `ImageInfo::total_bytes_buffer()` (used by `TiffDecoder::redistribute_limits`) computes the *original* TIFF color type's byte size (CMYK8 = 4 bytes/pixel), strictly larger than `layout.total_bytes()`'s RGB8 (3 bytes/pixel) for the same CMYK-source image - a genuine "intermediate wider color format before a final narrowing conversion" case, the exact shape the candidate's framing predicted. Every other decoder's internal scratch allocation checked in candidate 037's own sweep was either absent or exactly output-sized, not larger. |
| Code reading: `TiffDecoder::set_limits`/`redistribute_limits`, and `ImageReader::set_limits`/`decode_to_dynimage` in `src/io/image_reader_type.rs`, tracing how the caller's declared `max_alloc` reaches both trackers | Same checkout | Found `ImageReader::set_limits` clones the caller's full, undiminished `Limits` (including `max_alloc`) into `decoder.set_limits(...)` *and* keeps its own separate copy in `self.limits`. `decode_to_dynimage` later reserves `layout.total_bytes()` (the RGB output) from *its own* copy only - it never informs the decoder that this amount is no longer "available". `TiffDecoder::redistribute_limits` computes its own internal `decoding_buffer_size` cap purely from the raw `max_alloc` it was handed at `set_limits` time, with no knowledge that `layout.total_bytes()` worth of that same budget is concurrently spent on the caller's own output buffer. Two independent trackers, one shared numeric ceiling, no coordination between them. |
| Real end-to-end probe: `TiffDecoder::new`/`prepare_image`/`set_limits`/`read_image` (the real, documented public `ImageDecoder` API) against a real, well-formed CMYK8 TIFF (generated via the `tiff` crate's own `TiffEncoder`, not hand-crafted adversarial bytes), sweeping `max_alloc` across the boundary between "covers only the RGB output" and "covers the true combined live memory" | debug build, `cargo run --example`, 1000x1000 CMYK8 source (RGB output 3,000,000 bytes, CMYK internal requirement 4,000,000 bytes) | `read_image` succeeds (`Ok`) at `max_alloc = 5,500,000` and `7,000,000` - both strictly less than the true combined live memory (3,000,000 + 4,000,000 = 7,000,000, or at 7,000,000 exactly, still with zero margin for the `tiff` crate's own separate IFD/tag intermediate-buffer needs) - and only starts failing safely once `max_alloc` drops below roughly 4.3-5.5M (below the CMYK requirement alone, plus the external crate's own fixed parsing overhead). Reproduced deterministically, 3/3 runs. |

## Status

**Confirmed**.

Reproduces deterministically, 3/3 runs, debug and release builds, real `TiffDecoder` public API
against a real, well-formed CMYK8 TIFF file (produced by the `tiff` crate's own encoder).

**Fully reproducible**: **Yes**. The repro constructs `TiffDecoder` from a real, valid CMYK8 TIFF
file and drives it entirely through the real, documented public `ImageDecoder` API (`set_limits`,
`prepare_image`, `read_image`) - a single-participant finding (a caller configuring the `Limits`
it itself owns, then calling that same decoder's own public method with a real file), so
`CONFIRMATION.md` section 11's "real second participant" requirement doesn't add anything beyond
"run the real public API end-to-end against a real external artifact", which this repro does.

## Class

security

## Root cause

`Limits::max_alloc`'s doc comment states it is "the maximum allowed sum of allocations allocated
by the decoder at any one time... the bytes required to store the output image count towards this
value" - i.e. a single, shared budget that both the caller-supplied output buffer and any
decoder-internal scratch buffer are meant to draw from together.

For TIFF, that shared budget is implemented as *two independent trackers* that are never
reconciled:

1. `ImageReader`'s own `self.limits: Limits` (in `src/io/image_reader_type.rs`), which
   `decode_to_dynimage` reduces by `layout.total_bytes()` (the RGB output size) via
   `self.limits.reserve(...)` immediately before calling `read_image`.
2. `TiffDecoder`'s own `self.limits: tiff::decoder::Limits` (in `src/codecs/tiff.rs`), populated
   at `set_limits` time with the caller's *raw, undiminished* `max_alloc` value, then redistributed
   in `redistribute_limits()` into `decoding_buffer_size`/`intermediate_buffer_size` for the
   external `tiff` crate's own internal enforcement (`decoder/mod.rs`'s
   `size > limits.decoding_buffer_size / size_of::<T>()` check, confirmed to correctly hard-cap
   the CMYK-sized internal buffer at that value - the external crate does *not* over-allocate
   beyond what it was told).

Both trackers are checked against the *same* numeric `max_alloc` ceiling independently. Tracker 1
never tells tracker 2 that `layout.total_bytes()` worth of the shared budget is already
committed to the caller's own `buf` (already allocated before `read_image` is even called), so
tracker 2 can legitimately allow its own internal CMYK-sized scratch buffer up to the *full* raw
`max_alloc` - on top of, not carved out of, what tracker 1 already reserved. Combined live memory
during a CMYK TIFF decode can therefore reach `layout.total_bytes()` (RGB, tracker 1) +
`info.total_bytes_buffer()` (CMYK, tracker 2, up to `max_alloc`) `= 7/3` of `layout.total_bytes()`
for CMYK8->RGB8 in the worst case, while a caller who declared `max_alloc` per the doc's own stated
contract (enough to cover "the bytes required to store the output image", plus whatever margin
they judged sufficient for decoder-internal work) can see that margin exceeded by up to
~33% without either tracker's own internal check ever firing.

This is not a case of no accounting at all (unlike candidate 036's AVIF `aligned_store` or
candidate 038's pre-fix OpenEXR `rgba_channels` scratch buffer, both of which had *zero*
`Limits` awareness) - both TIFF-side trackers *do* enforce their own respective ceiling correctly
in isolation. The bug is specifically that neither ceiling accounts for what the other tracker has
already spent from the same declared budget.

## Introduced by

139e8092 "tiff: Support CMYK images (#2075)" (2023-12-25) - this commit changed the `max_intermediate_alloc` computation in `TiffDecoder::set_limits` from `max_alloc.saturating_sub(self.total_bytes())` (correctly carving out the RGB output buffer already reserved by `ImageReader`) to `max_alloc.saturating_sub(self.total_bytes_buffer())` (self-referentially subtracting the newly-added internal CMYK buffer's own size instead), which is the exact double-accounting defect described above.

## The fix

Added `ImageInfo::total_bytes_output()` (mirrors the existing `total_bytes_buffer()`, but computes
the size of the *final, converted* output buffer using `self.color_type`, the same value
`layout.total_bytes()` is ultimately derived from) and subtracted it from the local `max_alloc`
at the top of `redistribute_limits()`, before that function computes how much headroom is left for
`decoding_buffer_size`/`intermediate_buffer_size`. This makes `TiffDecoder`'s own internal budget
correctly reflect "what's left after the output buffer", regardless of whether the caller drives
this decoder through `ImageReader` (which separately, redundantly reserves the same amount) or
calls `TiffDecoder::set_limits`/`read_image` directly (where the output buffer isn't tracked by
any other `Limits` instance at all, so this fix is the *only* enforcement of the documented
contract in that mode).

- Re-ran `repro/` (`cargo run`, and again via `just test` against a fresh clone) against the
  patched tree: no longer reproduces, 3/3 runs (see `output.txt`, "AFTER FIX" section) -
  `read_image()` now returns `Err(Limits(LimitError { kind: InsufficientMemory }))` at the same
  `max_alloc = 5,500,000` that previously succeeded.
- Verified the boundary moved to the correct place, not just past the one probed value: a
  dedicated in-tree sweep (`examples/tmp_tiff_cmyk_probe.rs`, not part of the packaged `repro/`)
  across `max_alloc` from 3,000,000 up to `u64::MAX/2` shows the fixed tree only succeeds once
  `max_alloc >= 8,000,000` (comfortably above the true combined requirement of 7,000,000 plus the
  `tiff` crate's own fixed intermediate/IFD parsing overhead), whereas the unfixed tree already
  succeeded at 5,500,000 - see `output.txt`'s threshold-sweep section for both sides.
- A normal (non-CMYK) TIFF decode is unaffected: `total_bytes_output()` only ever differs from
  `total_bytes_buffer()` for `Cmyk8`/`Cmyk16` original color types (the only cases
  `total_bytes_buffer()` special-cases); every other color type already has
  `total_bytes_buffer() == total_bytes_output()`, so the new subtraction is `0` and
  `redistribute_limits()`'s behavior is unchanged for them.
- Project test suite (`cargo test --lib`, default features): **331 passed / 0 failed before, 331
  passed / 0 failed after** - no regressions, confirmed by reverting only the fixing hunk
  (`git stash push -- src/codecs/tiff.rs`) and rerunning both the test suite and `repro/` against
  the reverted tree before restoring it (`git stash pop`).

## What's not done / open questions

- This candidate only checked TIFF, per the specific "internal buffer larger than
  `layout.total_bytes()`" bug shape it set out to find. Candidate 037's own sweep already
  established that PNG/JPEG/GIF/WebP/ICO/HDR/OpenEXR/PNM/AVIF have no such larger-than-output
  internal buffer at all (either no allocation, or exactly output-sized) - this candidate re-read
  those `read_image` implementations too and found nothing new to add there, so the "larger than
  output" shape is now believed to be TIFF-specific in this codebase, but that wasn't independently
  re-verified against every color-type branch of every decoder (e.g. BMP's palette-expansion paths
  weren't specifically re-checked for this exact shape, only grepped in earlier candidates for a
  different bug class).
- Only TIFF's CMYK8/CMYK16 branches were exercised end-to-end; TIFF's `YCbCr8` branch also has
  `original_color_type != color_type` (both 3 bytes/pixel though, so `total_bytes_buffer() ==
  total_bytes_output()` there - not the same bug, not separately probed for a different kind of
  discrepancy) and the `L1` branch has a *smaller* internal buffer (packed bits), not a larger one
  - neither was exercised via the standalone `repro/`.
- The fix subtracts the output size unconditionally inside `redistribute_limits()`, which runs
  both from `set_limits` and from `reset_info_from_current_image` (i.e. also for callers who never
  call `set_limits` at all and rely purely on the trait-default no-limits behavior) - verified this
  doesn't panic or misbehave when `max_alloc` is `usize::MAX`-derived (the no-limits default),
  since `saturating_sub` never underflows, but a multi-image TIFF stream (`next_image`) advancing
  through frames of different color types wasn't separately exercised against this fix.
