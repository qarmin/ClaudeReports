# `ImageLoaderBMP`: unsupported bits-per-pixel value leaves the index buffer uninitialized, then used as palette indices (heap-buffer-overflow read)

Found by the GDScript binary-format fuzz loop (`Image.load_bmp_from_buffer()` on byte-mutated seeds), ASan editor
build, seed 1. The crashing mutation set `bmp_bit_count` to 0.

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of a 2x2 24-bit BMP | ASan editor build `b8420aa3a6`, seed 1 | ASan heap-buffer-overflow READ, `image_loader_bmp.cpp:196` |
| Generated repro (`repro/project/repro.gd`) with `bit_count` 0, 3, 5, 7 | unfixed build | ASan heap-buffer-overflow at the same line, every value, 10/10 for 5. Valid 1/2/4/8/16-bit files load fine |
| Fix verification | `patch.diff`, rebuilt | 0, 3, 5, 7 -> `Parse error`, no ASan report; 1/2/4/8/16 and a real 24-bit 2x2 BMP still load with the right pixels |

## Status

**Confirmed.** Deterministic, 10/10 ASan reports before, none after.

**Fully reproducible**: Yes - real `Image.load_bmp_from_buffer()` -> `ImageLoaderBMP::load_image()` on an in-memory BMP file.

## Class

`memory-safety` (heap out-of-bounds read of the palette buffer, plus use of uninitialized memory as pixel indices)

## Root cause

`modules/bmp/image_loader_bmp.cpp`: for `bmp_bit_count <= 8` `load_image()` allocates a palette of `1 << bit_count`
entries and `convert_to_image()` takes the "indexed" path (`data_len = width * height`, `data.resize()` without
initialization). The per-pixel `switch (bits_per_pixel)` only handles 1, 2, 4, 8, 16, 24 and 32, so for 0, 3, 5, 6, 7 no
index is ever written. The palette expansion loop then does `pal[write_buffer[i] * 4 + k]` with
uninitialized `write_buffer[i]` values up to 255 against a palette of only `1 << bit_count` (1, 8, 32 or 128) entries.

## Introduced by

6484da5721 Import 4/1 bit bmp images (introduced `bit_count <= 8` -> `1 << bit_count` palette sizing; best-effort `git log -S`)

## The fix

`patch.diff` (+13/-0): reject any `bmp_bit_count` other than 1, 2, 4, 8, 16, 24, 32 right after reading it, returning
`ERR_FILE_CORRUPT`. Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone; `repro/run.sh` run against the unfixed and fixed binaries from the main tree.
Other BMP header problems seen while reading but **not** investigated: `bmp_file_size < bmp_file_offset` makes
`bmp_buffer_size` wrap to ~4 GB (uint32 subtraction), the palette is read from the file with the header's own size without
checking the file length, and `width * height * 4` is computed in `int` (`data_len`).
