# `ImageLoaderTinyEXR::load_image()` heap-buffer-overflow when an EXR has an `A` channel but no `R`/`Y` channel

Found by a mutational fuzz loop written in GDScript (`Image.load_exr_from_buffer()` on byte-mutated
seeds from `Image.save_exr_to_buffer()`), run under the ASan editor build. Seed 1, iteration < 3000
crashed; the crashing file had its `R` channel name mutated to `0x7f`.

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz (GDScript, `tmp/bh/fz.gd`, not kept) over `load_png/jpg/webp/exr/svg_from_buffer`, `bytes_to_var[_with_objects]` | ASan editor build `b8420aa3a6`, seed 1, 3000 iterations per target | png, jpg, webp, var, varfull clean; **exr crashed** |
| Hand-built minimal input (`repro/project/repro.gd`) | `save_exr_to_buffer()` of a 7x5 image, channel `R` renamed to `X` | ASan heap-buffer-overflow at `image_loader_tinyexr.cpp:184`, 10/10 runs identical |
| Fix verification | one-line fix (`patch.diff`), rebuilt ASan editor | repro now returns `Parse error` (ERR_FILE_CORRUPT), no ASan report |

## Status

**Confirmed.** Deterministic (no threads/timing): 10/10 runs of `repro/run.sh` against the unfixed build
give the identical ASan report; the fixed build gives none.

**Fully reproducible**: Yes - the real `Image::load_exr_from_buffer()` entry point is called on a real EXR
file produced by the engine's own exporter with one channel renamed (no hand-faked second participant;
the same loader path is used when importing a `.exr` file).

## Class

`memory-safety` (heap out-of-bounds read of `tile.images[-1]`, then use of the garbage pointer as a pixel source)

## Root cause

`modules/tinyexr/image_loader_tinyexr.cpp`, `load_image()`: channel indices `idxR/G/B/A` start at -1 and
are set from channel names. The `idxB != -1`, `idxG != -1` and fall-through branches each guard with
`ERR_FAIL_COND_V(idxR == -1, ERR_FILE_CORRUPT)`, but the first branch (`idxA != -1`) does not. For a file
with channels like `A,B,G` (no `R`, no `Y`), the RGBA branch is taken and the loop does
`tile.images[idxR]` with `idxR == -1` (line 184), reading before the `images` array and then dereferencing
the garbage `float *` for every pixel.

Not covered by this fix / noticed but not pursued: in the RGBA branch a file with `A` plus `R` only (or
`R,G,A` without `B`) is accepted; the missing channels are skipped when writing, so the output pixels are
packed misaligned and part of the 4-channel buffer is left uninitialized. Not memory-unsafe, but a logic
problem in the same branch. The same class of check was not audited in other image loaders.

## Introduced by

e3eacbc8a6 Make EXR import format support all depths and channel configurations (the `idxR` guards were
added to the other branches then; the `idxA` branch, from 08f71baca5 Fix loading EXR with alpha channel,
was left without one).

## The fix

`patch.diff` (+1/-0): `ERR_FAIL_COND_V(idxR == -1, ERR_FILE_CORRUPT);` at the top of the `idxA != -1` branch.
Applied to the main tree, rebuilt (`scons ... use_asan=yes`), verified, reverted (`git checkout`).

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`, needs
network/IPv6 in this sandbox, unrelated); `--test-case="*Image*,*EXR*"`: 27/27 pass. A baseline full-suite run
crashed in the mbedtls `CryptoKey` tests (file-access error, environment dependent, in a test directory
shared with the earlier run) before reaching the end, so no clean full-suite baseline number exists.

## What's not done / open questions

`just test` was not run end-to-end from a fresh clone (each build is a full scons ASan build); `git apply
--check patch.diff` against a fresh checkout of `b8420aa3a6` succeeded, and `repro/run.sh` was run against
both the unfixed and fixed binaries built from the main tree.
