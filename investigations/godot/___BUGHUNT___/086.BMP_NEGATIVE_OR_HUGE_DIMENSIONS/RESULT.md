# `ImageLoaderBMP`: 32-bit width/height from the header are used unchecked (`width = 0xffffffff` crashes `convert_to_image`)

Found by the GDScript binary-format fuzz loop (`Image.load_bmp_from_buffer()`), seed 5, on a build that already had candidate 078's fix: the crashing 70-byte file
had its width field set to 0xffffffff. Related to, but independent from, candidate 078 (bit count).

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of a 2x2 24-bit BMP | build with 073-085 fixes, seed 5 | SIGSEGV in `ImageLoaderBMP::convert_to_image` |
| Generated repro (`repro/project/repro.gd -1 2`): 24-bit BMP, width -1 | unfixed build | SIGSEGV 10/10 (exit 134). Widths/heights 0, 2/-2, 65536x65536, 32768x32768 already return "Parse error" on the unfixed build (they trip the later file-size check); 2x2 and 1x1 load |
| Fix verification | `patch.diff`, rebuilt | -1, 0, -1x-1, 2x-2, 65536x65536: "Parse error", no crash; 1x1, 2x2 and the real 24-bit seed still load with the right pixel |

## Status

**Confirmed.** Deterministic, 10/10 crashes before, 0/10 after.

**Fully reproducible**: Yes - real `Image.load_bmp_from_buffer()` on an in-memory BMP.

## Class

`crash` (integer overflow in size arithmetic -> bad index)

## Root cause

`modules/bmp/image_loader_bmp.cpp`: `bmp_width`/`bmp_height` are `uint32_t` read straight from the header. `convert_to_image()` converts them to `size_t`, stores `width * height (* 4)` in an `int data_len`, and computes
`line_width = ((width * bits_per_pixel + 7) / 8 + 3) & ~3` in `uint32_t`; for `width = 0xffffffff` those wrap, the file-size sanity check (`line + line_width > end_buffer`) is evaluated with the wrapped numbers and
passes, and the pixel loop then writes/reads far outside the buffers. (A negative `height` is legal BMP - top-down rows - but is not supported and ends up as a huge unsigned value; it already fails the later check.)

## Introduced by

not determined (the dimension handling dates from the initial BMP importer; `git log -S` only reaches the 2019 rewrite 6484da5721 Import 4/1 bit bmp images, which did not introduce the missing check)

## The fix

`patch.diff` (+3/-0): reject `width == 0`, `height == 0`, or `width * height > INT32_MAX / 4` (64-bit product) with `ERR_FILE_CORRUPT` right after the header is read.
Applied to the main tree, rebuilt, verified, reverted.

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone; `repro/run.sh` run against the unfixed and fixed binaries from the main tree. Top-down (negative height) BMPs are still rejected, not supported.
The exact faulting statement for width 0xffffffff was not pinned down line by line (the crash backtrace is inside `convert_to_image`, no ASan report).
