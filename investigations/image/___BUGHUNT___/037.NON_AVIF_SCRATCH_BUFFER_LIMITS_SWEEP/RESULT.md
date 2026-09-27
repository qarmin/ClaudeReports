# NON_AVIF_SCRATCH_BUFFER_LIMITS_SWEEP

Follow-up sweep from `IDEAS.md`'s `AVIF_10BIT_SCRATCH_BUFFER_PATTERN_SWEEP` row: candidate 036
found `AvifDecoder::read_image`'s 10/12-bit path allocates a same-sized scratch `Vec<u16>`
(`aligned_store`) as a `bytemuck::try_cast_slice_mut` alignment fallback, without accounting for
it against `self.limits` (`max_alloc`). This candidate checks whether the same shape - a
decoder-owned allocation that duplicates the caller-supplied output buffer (or an equivalently
large scratch buffer) as a fallback/copy step, left unaccounted against `Limits` - exists
anywhere else in `src/codecs/` (PNG, JPEG, TIFF, GIF, WebP, ICO, HDR, OpenEXR, PNM).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static grep sweep of `src/codecs/` for the mechanical fingerprints of this pattern: `bytemuck::try_cast_slice(_mut)`/`cast_slice(_mut)`/`pod_collect_to_vec`/`PodCastError`/`Cow::Owned`, plus any `vec![0...; buf.len() ...]`/`buf.to_vec()`/`.to_owned()` scratch allocation sized off an output buffer or frame | Static read of current `main` (`44ce9226`) | Found candidate call sites in `png.rs` (`blend_pixel_bytes`), `tiff.rs` (`u8_slice_as_pod`, plus 8 `cast_slice` copies in `ycbcr`/CMYK handling), `hdr/decoder.rs` (`to_hdr`), `openexr.rs` (`unaligned_bytes.copy_from_slice`), `avif/encoder.rs` (its own `try_cast_slice`/`PodCastError` fallback, structurally identical to the already-fixed decoder one), `gif.rs` (`data.to_owned()` in the encoder), and test-only `vec![0; buf.len()...]` uses in `pnm/mod.rs` and `ico/decoder.rs`. |
| Manual code reading of every hit's surrounding function, and of every decoder's `set_limits`/`read_image` for how each format's *decode*-path scratch allocations are (or aren't) accounted, cross-checked against which of the 7 decoders that implement `set_limits` at all (JPEG, ICO, TIFF, GIF, AVIF, PNG, HDR) each hit belongs to | Same checkout | `png.rs::blend_pixel_bytes` uses `cast_slice_mut` on the caller's own already-owned buffer (no allocation at all - not a match). `hdr/decoder.rs`'s `scanline` allocation (one row, not a duplicate of the whole output) is correctly reserved via `set_limits`'s `scanline_space` reserve. `gif.rs::read_image`'s "frame doesn't match logical screen" branch allocates a scratch `frame_buffer` sized off the frame - correctly wrapped in `self.limits.reserve_usize(buffer_size)?` immediately before the `vec![0; buffer_size]`. `tiff.rs::u8_slice_as_pod`'s `pod_collect_to_vec` fallback (structurally the closest match to AVIF's `aligned_store` - a same-sized duplicate allocation on a `PodCastError::TargetAlignmentGreaterAndInputNotAligned` alignment fallback) and `avif/encoder.rs`'s equivalent fallback are both reachable only from `write_image`/`encode`, i.e. the **encoder** side of each format, not the decoder side. `openexr.rs` has no `set_limits` override at all (uses the trait default, which does not add any `max_alloc` reservation for this decoder), so its `unaligned_bytes` copy is not "unaccounted against a Limits check that should have caught it" - there is no such check anywhere in this decoder to begin with. |
| Confirmed by reading `src/io/encoder.rs`'s `ImageEncoder` trait definition directly: it has no `set_limits`/`Limits`-typed method or field anywhere, unlike `ImageDecoder` | Same checkout | `ImageEncoder` never exposes any `Limits`/`max_alloc` mechanism to a caller in this crate at all - a caller who owns a `buf: &[u8]` already in memory and passes it to `write_image` has no `Limits` promise available to violate, since none exists on that path. This rules out `tiff.rs::u8_slice_as_pod` and `avif/encoder.rs`'s matching fallback as instances of the same bug class as candidate 036 (a `Limits`/`max_alloc` enforcement gap) - there is no enforcement mechanism there to have a gap in. |

## Status

**Not reproducible.** Every decoder-side (i.e. `set_limits`-bearing) hit found by the sweep either
turned out not to allocate at all (`png.rs`'s `cast_slice_mut` writes directly into the caller's
own buffer), or already correctly reserves the scratch allocation against `self.limits` before
making it (`hdr/decoder.rs`'s per-scanline buffer, `gif.rs`'s mismatched-frame scratch buffer).
The two structurally-closest matches to candidate 036's `aligned_store` shape - `tiff.rs`'s
`u8_slice_as_pod` fallback and `avif/encoder.rs`'s own equivalent - both live exclusively on the
**encoder** side of their formats, where this crate's `ImageEncoder` trait has no `Limits`/
`max_alloc` mechanism at all (confirmed by reading the trait definition), so there is no
enforcement being bypassed there. No fuzzing/miri/sanitizer pass was needed - this is purely a
static presence-or-absence check of a `limits.reserve*` call adjacent to each allocation, and
every decoder-reachable allocation site in `src/codecs/{png,jpeg,tiff,gif,webp,ico,hdr,openexr,
pnm}.rs`/`decoder.rs` was read directly.

## Root cause

N/A (no reproduction). The suspect area is safe: unlike `AvifDecoder::read_image`'s 10/12-bit
path, none of the other decoders that implement `ImageDecoder::set_limits` have an unaccounted
scratch-duplicate-of-the-output-buffer allocation in their `read_image`/frame-decode path. The
only two code sites with the exact same *shape* as AVIF's fixed bug (a `bytemuck`
alignment-fallback `Vec` the same size as an externally-visible buffer) are both on the encoder
side, where this crate simply doesn't offer any `Limits` contract to violate, so the AVIF bug's
specific class ("a caller-set `max_alloc` is silently bypassed by a decoder-owned scratch
allocation") does not recur elsewhere in this codebase's own `src/`.

## What's not done / open questions

- `src/codecs/webp/decoder.rs` delegates all pixel decoding to the external `image_webp` crate
  (not vendored in this repo's `src/`); any equivalent scratch-buffer pattern inside that crate's
  own code was out of scope here, same as `image_webp`/`dav1d`'s own internals were out of scope
  for candidate 036 (the underlying decode buffers there are pre-`Limits`-existence anyway, per
  036's own open-questions note about `dav1d`'s `Picture` memory).
- `openexr.rs` has no `set_limits` override at all - it silently accepts the default trait
  behavior (essentially no `max_alloc` enforcement for this format). This is a broader gap than
  the one this candidate was scoped to check (it isn't a case of "an allocation slips past an
  otherwise-present check" - there is no check to slip past for this decoder in general), so it
  was noted but not investigated further as its own candidate.
- If this crate's `ImageEncoder` trait is ever given a `Limits`/`max_alloc` mechanism in the
  future, `tiff.rs::u8_slice_as_pod` and `avif/encoder.rs`'s matching fallback would become the
  first two sites needing an accounting call added, by direct analogy with candidate 036's fix -
  worth a forward note for whoever does that work, not an actionable bug today.
