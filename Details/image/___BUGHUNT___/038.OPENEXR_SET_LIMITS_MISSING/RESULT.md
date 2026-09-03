# OPENEXR_SET_LIMITS_MISSING

`src/codecs/openexr.rs`'s `OpenExrDecoder` has no `ImageDecoder::set_limits` override at all
(uses the trait default), unlike PNG/JPEG/TIFF/GIF/AVIF/ICO/HDR. Flagged in `IDEAS.md` as a
candidate structural gap: does `max_alloc`/dimension limits enforcement actually do nothing for
this format?

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `ImageDecoder` trait default `set_limits`, `Limits::check_layout_dimensions`/`reserve`, and `ImageReader::decode_to_dynimage`'s own limit handling in `src/io/image_reader_type.rs` | debug build | Found the *default* trait `set_limits` already calls `prepare_image()` + `Limits::check_layout_dimensions`, so `max_image_width`/`max_image_height` ARE enforced for OpenEXR even without a custom override. Also found `ImageReader::decode_to_dynimage` (the implementation behind the crate's main public `ImageReaderOptions::decode()`/`ImageReader::decode()` entry points) independently calls `self.limits.reserve(layout.total_bytes())` *before* calling `read_image` at all, for every decoder uniformly - so `max_alloc` for the *output* buffer is also centrally enforced there, regardless of a decoder's own `set_limits`. |
| Real end-to-end probe #1: `ImageReaderOptions::open(real .exr test file).limits(Limits{max_alloc: Some(16)}).decode()` and a second run with `max_image_width/height: Some(1)`, via a throwaway `cargo run --example` against this crate's own public API | debug build, real `tests/images/exr/cropping - uncropped original.exr` (1920x1920) | Both correctly rejected (`LimitError::InsufficientMemory` / `LimitError::DimensionError`) - confirms the candidate's premise is **wrong** for the crate's main public decode path: `ImageReaderOptions`/`ImageReader` already protect OpenEXR via limit checks that live outside the decoder, not inside it. |
| Real end-to-end probe #2: construct `OpenExrDecoder` directly (bypassing `ImageReader`), call `decoder.set_limits(Limits{max_alloc: Some(0), ..no_limits()})` (the real, documented `ImageDecoder::set_limits` API contract), then `decoder.read_image(buf)` with a buffer already sized to `layout.total_bytes()` | debug build, same real test file | `set_limits` returned `Ok(())` (no rejection - `max_alloc` is invisible to the trait-default check), and `read_image` *succeeded* despite `max_alloc: Some(0)`, silently allocating a second, decoder-owned `~59MB` `vec![0_f32; ...]` scratch buffer (the `rgba_channels` closure's own allocation inside `read()`) with zero `Limits` accounting - reproduced 3/3 runs. This is the real, narrower gap: identical shape to candidate 036's AVIF `aligned_store` finding, not "no enforcement at all". |

## Status

**Confirmed** (narrower than the original candidate framing - see "Root cause" below for what
turned out to be already-protected vs. genuinely missing).

Reproduces deterministically, 3/3 runs, debug build, real `OpenExrDecoder` public API against a
real EXR file from this crate's own test corpus (`tests/images/exr/cropping - uncropped
original.exr`).

**Fully reproducible**: **Yes**. The repro constructs `OpenExrDecoder` from a real embedded EXR
file (this crate's own test corpus) and drives it entirely through the real, documented public
`ImageDecoder` API (`set_limits`, `prepare_image`, `read_image`) - a genuine external artifact
handed to the decoder's real public entry point, with no second participant to fake (per
`CONFIRMATION.md` section 11's explicit carve-out for "a malformed file handed to a parser's real
public entry point").

## Class

security

## Root cause

The original candidate's framing ("max_alloc/dimension limits are effectively not enforced at all
for OpenEXR") is **not accurate** for the crate's main, documented decode path
(`ImageReaderOptions::decode()` / `ImageReader::decode()`):

- `ImageDecoder`'s trait-*default* `set_limits` (used automatically since `OpenExrDecoder` doesn't
  override it) already calls `self.prepare_image()` and `Limits::check_layout_dimensions`, which
  enforces `max_image_width`/`max_image_height` even without a decoder-specific implementation.
- `ImageReader::decode_to_dynimage` (`src/io/image_reader_type.rs`, the code behind the main
  public decode entry points) independently calls `self.limits.reserve(layout.total_bytes())`
  *before* ever calling `read_image`, for every decoder uniformly, regardless of whether that
  decoder implements its own `set_limits`. Since OpenEXR's internal scratch allocation is exactly
  `display_window.size.area() * channel_count` `f32`s - the same size as `layout.total_bytes()`,
  which was already reserved - this call transitively protects OpenEXR's real end-to-end decode
  path from the huge-header-declared-dimensions DoS scenario the original candidate worried about.

The real, narrower gap is specific to *direct* decoder usage (constructing `OpenExrDecoder`
yourself and calling `ImageDecoder::set_limits`/`read_image` directly, bypassing `ImageReader`) -
a usage pattern the `ImageDecoder` trait's own doc comment explicitly documents as supported
(`configure = "set_limits"` in the decoding-sequence grammar). In that mode:

- `OpenExrDecoder` has no `limits` field and no custom `set_limits`, so the trait-default
  `set_limits` accepts any `max_alloc` value (including `Some(0)`) with `Ok(())` and silently
  discards it - there is nowhere for the value to be remembered.
- `read_image` (`src/codecs/openexr.rs:134`) allocates its own `f32` scratch buffer via the `exr`
  crate's `rgba_channels(move |_size, _channels| vec![0_f32; display_window.size.area() *
  channel_count], ...)` callback, sized directly off header-declared dimensions, then later copies
  it into the caller-supplied `unaligned_bytes` via `copy_from_slice`. This is a genuine *second*
  allocation, the same size as the output buffer, with zero `Limits` awareness - identical in
  shape to candidate 036's AVIF `aligned_store` finding (`___BUGHUNT___/036.AVIF_10BIT_UNALIGNED_BUFFER_ALLOC_UNACCOUNTED/RESULT.md`).
- Note that the *caller* must already have allocated a buffer the size of `layout.total_bytes()`
  to call `read_image` at all (its own responsibility, same as every other decoder) - the bug is
  specifically the *additional*, decoder-owned scratch buffer duplicating that size internally,
  not the first allocation.

## Introduced by

dba7b604 "Add decoding limits support" (2021-03-05) - this commit introduced the crate-wide `Limits`/`ImageDecoder::set_limits` mechanism (including the trait-default `set_limits` that silently accepts any `max_alloc`) without ever touching `src/codecs/openexr.rs`; the OpenEXR decoder's `read_image` scratch allocation itself predates this and was never retrofitted with `Limits` awareness in any later commit, so no single commit regressed previously-enforced behavior - it is an omission rather than a regression.

## The fix

Added a `limits: crate::Limits` field to `OpenExrDecoder`, defaulted to `Limits::no_limits()` in
both constructors (matching the trait doc's stated default of "no limits", and the majority
convention among sibling decoders e.g. JPEG/GIF). Implemented `ImageDecoder::set_limits` for
`OpenExrDecoder` (identical body to the trait default, plus storing the value), and added a
`self.limits.reserve_usize(...)` call in `read_image` immediately before the internal scratch
allocation, sized identically to what the `rgba_channels` closure is about to allocate
(`display_window.size.area() * channel_count * size_of::<f32>()`, using `saturating_mul` to avoid
overflow on adversarial header dimensions).

- Verified against a more generous limit (`max_alloc: Some(200 MiB)`) that a normal decode still
  succeeds unaffected (probe run, not part of the packaged `repro/`, output not separately saved).
- Re-ran `repro/` against the patched tree: no longer reproduces, 3/3 runs (see `output.txt`,
  "AFTER" section) - `read_image` now returns `Err(Limits(LimitError { kind:
  InsufficientMemory }))` instead of silently allocating.
- Project test suite (`cargo test --lib --features exr`, which matches this crate's
  `default-formats`): **331 passed / 0 failed before, 331 passed / 0 failed after** - no
  regressions, including the pre-existing `codecs::openexr::test::compare_cropped` test.

## What's not done / open questions

- This candidate's own repro only varies `max_alloc` (the value actually broken); it did not
  separately re-verify `max_image_width`/`max_image_height` behavior since candidate probe #1
  already showed those work correctly through the *default* trait behavior both before and after
  this patch (this patch's `set_limits` override has an identical dimension-check body to the
  trait default it replaces).
- Whether `ImageReaderOptions`/`ImageReader`'s own centralized `self.limits.reserve(layout.total_bytes())`
  check (independent of any decoder-level `set_limits`) is itself a load-bearing safety net for
  *other* decoders with similarly-shaped "no custom `set_limits`" gaps was not swept here - this
  candidate only concerned `openexr.rs`. Candidate 037's own sweep already covered the decoder-
  internal-allocation shape broadly and flagged `openexr.rs` specifically as its own follow-up;
  this candidate closes that follow-up.
- The `IDEAS.md` backlog note that spawned this candidate described the risk as reachable through
  "a caller's `ImageReaderOptions::limits(...)` ... call" specifically - that phrasing turned out
  to describe the *already-protected* path; the real risk is direct decoder API usage, which this
  `RESULT.md`'s "Root cause" section calls out explicitly so a future reader doesn't have to
  re-derive that distinction.
