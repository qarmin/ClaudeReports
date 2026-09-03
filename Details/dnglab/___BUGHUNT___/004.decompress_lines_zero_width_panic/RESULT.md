# Any TIFF/DNG-family decoder panics on `ImageWidth=0` (or `SamplesPerPixel=0`)

`rawler::decompressors::{decompress_lines_fn, decompress_lines, decompress_strips_fn,
decompress_strips}` are shared helpers used by *most* of the TIFF-family decoders in this crate
(DNG's own uncompressed/packed path, ARW, NEF, RW2 v6/v7, IIQ, and the whole
`decompressors/packed.rs` family of bit-depth decoders) to allocate the output image and fan the
actual decompression out across rows/strips with `rayon`. Each one takes `width` straight from a
caller that in turn read it straight from an untrusted file tag (`ImageWidth`, or `ImageWidth *
SamplesPerPixel`) with no minimum-value validation, then passes it as the chunk size to
`par_chunks_mut`/`par_chunks_exact_mut` - `HEURISTICS.md`'s "arithmetic on sizes/lengths/indices
derived from input" pattern. `rayon`'s (and `std`'s) chunking iterators panic outright with
`"chunk_size must not be zero"` if that computed size is 0.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo fuzz run` on the existing `fuzz_bmff_parse`/`fuzz_tiff_parse`/`fuzz_decode_full`/etc targets (same run that found candidate 003) | ~25 min, no seed corpus | One worker's log showed a `"chunk_size must not be zero"` panic, but a concurrency mistake in this session's own orchestration (multiple `cargo fuzz run` invocations sharing one working directory's `fuzz-0.log`/`fuzz-1.log`, see `../README.md`) made it impossible to tell which target/input actually produced it |
| Code reading of `decode_strips`/`plain_image_from_ifd` (`decoders/mod.rs`) given the panic message and the fuzzer's ambiguous log | - | Found `decode_strips` reads `width`/`height`/`cpp` directly from `TiffCommonTag::ImageWidth`/`ImageLength`/`SamplesPerPixel` with no validation and passes `width * cpp` into `decompress_lines_fn`, which uses it as `par_chunks_mut`'s chunk size |
| Standalone repro: a hand-built, spec-shaped uncompressed DNG with `ImageWidth=0` (via rawler's own low-level `formats::tiff::writer::TiffWriter`, decoded via the real, public `rawler::analyze::raw_to_srgb`) | debug build, 3 runs; release build, 1 run | Deterministic panic, 3/3 debug + 1/1 release, identical message/location each time |
| Boundary-value sweep after applying the fix: `(width=0,height=5)`, `(width=0,height=0)`, `(width=5,height=0)`, `(width=1,height=1)`, `(width=5,height=5)` | debug build | The two `width=0` cases are now clean errors, no panic. `(width=5,height=0)` surfaces a **different, pre-existing, unfixed** panic (`rawimage.rs:287`, `assertion failed: !image.is_empty()`) - not this candidate's bug, noted below. The two normal-size cases correctly hit the already-known, already-fixed-and-reverted candidate 001 bug (unrelated to this patch) |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after |

## Status

**Confirmed.** Reproduces 3/3 debug runs and 1/1 release run, fully deterministic. Not gated by
`overflow-checks`/build profile - `rayon`'s own chunk-size check panics unconditionally, same as
candidate 003.

**Fully reproducible**: **Yes**. `repro/` writes a real, spec-shaped DNG file to disk (differing
from a normal file only in its `ImageWidth=0` tag value) and decodes it through the unmodified,
public `rawler::analyze::raw_to_srgb` - the same function `dnglab analyze --srgb <file>` calls.
Single-participant case, no second participant to fake, per `CONFIRMATION.md` section 11.

## Class

`crash`.

## Root cause

`decode_strips` (`decoders/mod.rs:727-748`, used by `plain_image_from_ifd`, which the DNG decoder
and several others call):

```rust
let width = fetch_tiff_tag!(raw, TiffCommonTag::ImageWidth).force_usize(0);
...
let cpp = fetch_tiff_tag!(raw, TiffCommonTag::SamplesPerPixel).force_usize(0);
...
Ok(decompress_lines_fn(width * cpp, height, dummy, &|line, row| { ... })?)
```

`width` and `cpp` are read straight from file-controlled TIFF tags with `force_usize` (which
coerces whatever numeric value is present, including 0, with no minimum-value check). If either is
0, `width * cpp` is 0, which becomes the chunk size passed to
`.pixels_mut().par_chunks_mut(width)` inside `decompress_lines_fn` (`decompressors/mod.rs:143-155`).
`rayon`'s chunking panics unconditionally with `"chunk_size must not be zero"` whenever the chunk
size argument itself is 0 - regardless of whether the underlying slice is empty, and regardless of
build profile/overflow-checks. `decompress_lines`, `decompress_strips_fn`, and `decompress_strips`
have the identical shape (chunk size derived from `width` or `width * stripsize`, no zero-check),
and are reachable the same way from other decoders sharing these helpers.

## The fix

Added an explicit `width == 0` guard (and `width == 0 || stripsize == 0` for the two strip-based
variants) at the top of all four functions, returning a descriptive `Err` instead of proceeding to
the panicking `par_chunks_mut`/`par_chunks_exact_mut` call. This is the same fix shape in all four
places since they share the identical root cause. `decompress_chunked_fn` (the fifth helper in
this file, chunk-size-driven the same way) was left unfixed - see "What's not done".

Verified: `repro/` (`ImageWidth=0`) no longer panics and returns a descriptive error instead, in
both debug and release. Re-verified against a `width=0, height=0` variant (still cleanly caught by
the same guard) to confirm the fix isn't narrowly matched to the exact triggering byte values.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no existing test exercises a zero-width image, so this fix is
  additive coverage, not a regression risk.

## What's not done / open questions

- `decompress_chunked_fn` (`decompressors/mod.rs:284-294`) has the identical
  chunk-size-from-parameter shape (`par_chunks_mut(chunksize)`) but a different signature (returns
  `Ok` unconditionally, no `Result`-based error path for its closure) - fixing it the same way
  would need a small API change (giving it a fallible return), which felt like a bigger change to
  make speculatively without a confirmed reachable zero-`chunksize` caller. Not independently
  confirmed exploitable this session.
- A **separate, unfixed** bug was surfaced while boundary-testing this fix: `(width=5, height=0)`
  panics at `rawimage.rs:287` with `assertion failed: !image.is_empty()`, inside `RawImage`'s own
  constructor - a `height=0` (rather than `width=0`) image is caught by a different invariant
  entirely, downstream of this candidate's code. Not investigated further this session; the exact
  triggering input (`ImageWidth=5, ImageLength=0`) is recorded here for a future session.
- This is the same file-tag-controlled-dimension class of bug as candidate 001 (PPG demosaic) but
  at an earlier pipeline stage (decode, not develop/demosaic) - both stem from the project's
  decoders not validating `ImageWidth`/`ImageLength`/`SamplesPerPixel` against a sane minimum
  anywhere centrally. A single shared validation point (e.g. in `RawImage`'s own constructor, or
  in `decode_strips`/`decode_tiles` before dispatching to any decompressor) would likely close
  most of this bug class at once, rather than patching each downstream consumer separately - worth
  considering as a follow-up architectural fix, out of scope for this session's bounded patches.
