# `ImageLoaderTGA`: `int` pixel-offset overflow for large dimensions writes before the image buffer (SIGSEGV); truncated pixel data returns uninitialized memory as pixels

Found by the GDScript binary-format fuzz loop (`Image.load_tga_from_buffer()` on byte-mutated seeds), ASan editor
build, seed 1. The crashing 31-byte file declared 65535 x 32767 pixels.

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of a 2x2 24-bit TGA | ASan editor build `b8420aa3a6`, seed 1 | SIGSEGV in `ImageLoaderTGA::convert_to_image` (`TGA_PUT_PIXEL`, line 218) |
| Generated repro (`repro/project/repro.gd`): 18-byte header with 65535 x 32767, 300 bytes of pixel data | unfixed build | SIGSEGV 10/10 (with only 3 bytes of data it survives: the single write lands in mapped memory) |
| Related probe: 2x2 24-bit TGA with only 2 of 4 pixels present | unfixed build | `OK`, and the missing pixels contain uninitialized heap bytes (`get_pixel(1,1)` = (0.745, 0.745, 0.745)); fixed build returns `Parse error` |
| Fix verification | `patch.diff`, rebuilt | repro `Parse error`, 10/10; valid raw 24-bit and RLE 24-bit TGAs still load with identical pixels |

## Status

**Confirmed.** Deterministic, 10/10 crashes before, 0/10 after.

**Fully reproducible**: Yes - real `Image.load_tga_from_buffer()` -> `ImageLoaderTGA::load_image()` on an in-memory file.

## Class

`memory-safety` (signed integer overflow -> out-of-bounds heap write before the buffer; plus uninitialized read on truncated data)

## Root cause

`modules/tga/image_loader_tga.cpp`:

- `TGA_PUT_PIXEL` computes `int image_data_ofs = ((y * width) + x);` then indexes `image_data_w[image_data_ofs * 4 + k]`.
  `width`/`height` are up to 65535 (16 bit header fields), so `image_data_ofs * 4` overflows `int` once the image has more
  than 2^29 pixels; the (UB) wrapped negative offset writes before the start of the 8.6 GB `image_data` buffer.
- `buffer_size = (tga_header.image_width * tga_header.image_height) * pixel_size` multiplies two `uint16_t` promoted to
  `int`, which overflows as well (65535 * 65535).
- For uncompressed images the pixel-data length is never compared with `width * height * pixel_size`; the converter
  only stops when the index reaches `src_image_len` (the whole file length, not the pixel data length), so pixels past
  the real data are read from the uninitialized tail of `src_image` (the file was read to offset 0 of that buffer).

## Introduced by

871ceeb0fe TGA support based on SaracenOne's work.

## The fix

`patch.diff` (+8/-3): use `size_t` for the pixel offset and the dimension product, and reject uncompressed files whose pixel
data is shorter than `width * height * pixel_size` (`ERR_PARSE_ERROR`). Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone; `repro/run.sh` run against the unfixed and fixed binaries from the main tree.
RLE images with huge dimensions still allocate `width * height * pixel_size` bytes up front (up to ~17 GB) before
the RLE decoder rejects short input; `uncompressed_buffer.resize()` result is not checked. Not changed.
