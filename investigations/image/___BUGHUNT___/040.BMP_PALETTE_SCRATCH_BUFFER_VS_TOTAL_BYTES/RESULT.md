# BMP_PALETTE_SCRATCH_BUFFER_VS_TOTAL_BYTES

From `IDEAS.md`: does BMP's palette-expansion / bit-depth-upconversion decode path
(`src/codecs/bmp/decoder.rs`, the `indices` scratch buffer around line 1848 and its siblings in
the 16-bit/32-bit/full-byte read paths) allocate an internal buffer sized larger than
`layout.total_bytes()`, the same "decoder-internal scratch buffer bypasses the caller's declared
`max_alloc` budget" shape candidate 039 CONFIRMED for TIFF's CMYK handling?

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: every pixel-data read path in `src/codecs/bmp/decoder.rs` that allocates a scratch buffer during decode (`read_palettized_pixel_data`'s `indices`, `read_16_bit_pixel_data`/`read_32_bit_pixel_data`/`read_full_byte_pixel_data`'s `row_buffer` via `allocate_row_buffer`, `read_rle_data`'s `rle_indices_buffer`, `read_palette`'s palette `buf`) | Static read of current `main` (`44ce9226`) | Every one of these buffers is sized off a **single row** (`row_byte_length`/`row_data_len`/`total_row_len`, all `O(width)`) or a fixed 256-byte constant (`rle_indices_buffer`, palette `buf` capped at `MAX_PALETTE_SIZE * bytes_per_color`), and is allocated **once outside the per-row closure and reused** for every row via `with_rows_resumable`'s closure - never accumulated across rows/height. This is structurally different from TIFF's bug: TIFF's `image-tiff` internal buffer holds the **entire decoded image** in the wider CMYK format before the narrowing conversion (`O(width*height*4)` vs. output `O(width*height*3)`); BMP's scratch buffers never scale with `height` at all. |
| Code reading: `self.width`/`self.height` bounds and their relationship to `layout.total_bytes()` (`DecoderPreparedImage::new(self.width as u32, self.height as u32, color)` in `prepare_image`) | Same checkout | `self.width`/`self.height` are `i32`, hard-capped at `MAX_WIDTH_HEIGHT = 0xFFFF` (65535) during header parsing (`read_bitmap_info_header`, `read_bitmap_core_header`), and are the exact same values used both to compute `layout.total_bytes()` and every row-buffer's `row_byte_length`/`row_data_len` - no mismatch between the value `total_bytes()` is derived from and the value scratch-buffer sizing uses, unlike a scenario where an internal buffer is sized off a different (wider/original) representation than the final output color type. |
| Real end-to-end probe: constructed a real, well-formed 8bpp-palette BMP file via a byte-level BMP encoder (following the documented file/header/palette/pixel-data layout, not hand-crafted adversarial bytes) with `width = 65535, height = 1` (the worst case for the ratio of one-row scratch buffer to total output, since `total_bytes()` is smallest relative to a single row's size when `height = 1`), decoded via the real public `BmpDecoder::prepare_image`/`read_image` API, and compared the internal `row_byte_length` formula's value against `layout.total_bytes()` | debug build, `cargo run --example`, one-shot | `layout.total_bytes() = 196605` (65535 x 1 x 3 bytes/pixel RGB8 - `indexed_color` defaults to `false`, so an 8bpp palette source is expanded to RGB8 output by default), internal `indices` scratch buffer = `row_byte_length = 65536` bytes - **33% of** `total_bytes()`, not larger than it. `decoder.read_image()` succeeded and the assertion `row_byte_length <= total_bytes` held. |

## Status

**Not reproducible.** Code reading covered every scratch-buffer allocation in BMP's pixel-data
decode paths (palettized/16-bit/32-bit/full-byte/RLE/palette-table read functions) and found none
of them scale with image height the way TIFF's whole-image CMYK buffer did - every one is either a
fixed small constant or `O(width)` (at most one row), reused across the whole decode rather than
accumulated. A real end-to-end probe at the theoretically worst-case aspect ratio (`height = 1`,
maximizing the row-buffer-to-total-output ratio) confirmed the scratch buffer stays well under
`layout.total_bytes()` rather than exceeding it. No fuzzing/miri/sanitizer pass was needed - this
is a structural, not input-dependent, property (the buffer's size formula has no term that scales
with `height`, so no adversarial input can push it past `total_bytes()`, which does scale with
`height`), so the two approaches above (exhaustive static read + one concrete boundary-value real
run) are sufficient to rule this out.

## Root cause

N/A (no reproduction). The suspect area is safe, but for a different reason than TIFF's fix
addressed: BMP's decoder allocates its per-row scratch buffers as fixed-size buffers reused across
the entire decode loop (`with_rows_resumable`'s closure captures `&mut indices`/`&mut row_buffer`
by reference and calls `read_scanline` into the same allocation every row), rather than TIFF's
approach of decoding the whole image into one wide-format buffer up front. Since BMP's per-row
buffer size is bounded by `MAX_WIDTH_HEIGHT` (65535) and never multiplied by `height`, and
`layout.total_bytes()` (which the caller's `Limits::reserve` accounts for) is `width * height *
bytes_per_pixel`, the scratch buffer is smaller than `total_bytes()` for any image with `height >=
1` and only approaches (never exceeds) it in the degenerate `height = 1` case. Note: this
candidate did **not** check whether BMP's *absence* of any `ImageDecoder::set_limits` override at
all (it uses the trait's default, which checks dimensions but never enforces `max_alloc` against
any decoder-internal allocation) is itself a gap - that is a different bug shape (no accounting at
all, the class candidate 038/pre-fix OpenEXR and candidate 037's noted-but-out-of-scope OpenEXR
observation cover), not the "internal buffer bigger than `total_bytes()`" double-accounting shape
this candidate specifically set out to check, and BMP's own scratch buffers are small enough
(bounded by `MAX_WIDTH_HEIGHT`, not `max_alloc`-worthy on their own) that it would be a much
weaker finding than TIFF's even if pursued.

## What's not done / open questions

- BMP's complete absence of a `set_limits` override (unlike TIFF/GIF/JPEG/AVIF/ICO/PNG/HDR, all of
  which implement one) means `max_alloc` is never enforced against *any* BMP-internal allocation,
  including the ones checked here - they just happen to be too small (row-bounded, capped at
  65535) to matter much even if enforced. Added to `IDEAS.md` as a separate, narrower suspect: BMP
  joins the "no `set_limits` at all" list alongside OpenEXR (pre-038-fix), worth checking once as
  its own candidate for whether the *default* trait's dimension-only check is sufficient here or
  whether some other BMP-specific allocation (e.g. the ICC profile buffer at line ~1478, sized
  directly off a file-supplied `profile_size` with its own `vec_try_with_capacity` OOM guard, but
  not gated by any caller-declared `max_alloc`) is unaccounted.
- Only 8bpp palette-indexed decoding was exercised end-to-end via the real API; 1/2/4-bit
  palettized paths and the 16-bit/32-bit bitfield paths were checked by code reading only (their
  scratch-buffer formulas are all `O(width)`, same shape, so a dynamic probe of each was judged
  low-value after the 8bpp case already demonstrated the structural bound holds) - not re-verified
  with a real run per bit depth.
