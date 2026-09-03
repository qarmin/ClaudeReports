# What is under test, and how it is measured

This is a vendored dependency workspace, cloned locally (sibling of hayro) because a suspected
perf issue lived inside `vello_cpu`/`vello_common` rather than in hayro's own code - see hayro's
own `___OPTIMIZATIONS___/README.md` for how this was found (the pipeline breakdown that flagged
PNG encoding and unpremultiply as candidates).

## Workload

| mode/scene | what it stresses |
|---|---|
| `001.take_unpremultiplied_lut/bench/` | `Pixmap::take_unpremultiplied` alone, on a synthetic 4000x3000 premultiplied buffer |
| `002.png_compression_level/bench/` | `Pixmap::into_png`'s DEFLATE step alone, on real rendered output from the user's `img.pdf` (compression ratio is content-sensitive, so synthetic noise would not be representative here) |
| `003.png_compression_level_fastest/bench/` | Same harness as `002`, copied in unmodified (`Fast` vs. `Fastest` follow-up) |

## Measurement harness

Each candidate's `bench/` is a standalone crate with a real path dependency on the `vello_common`
crate under test (patched in place, then rebuilt) - see each candidate's own `RESULT.md` for the
exact invocation and iteration count. Each prints per-iteration timing and hash-checks/pixel-
compares output for correctness every run.

## Environment

AMD Ryzen 9 7900 (12-core). `taskset -c 2` for all single-core numbers. `perf` was not installed
(no root available to install it without asking) - profiling used in-code `Instant`+`AtomicU64`
phase timers instead, applied temporarily and reverted after capturing numbers.

`git-lfs` is not installed locally, so this clone's PNG snapshot test assets appear as spuriously
"modified" under plain `git status`/`git diff` (the LFS smudge/clean filters fail) - this is
environment noise, not a real change; use
`git -c filter.lfs.process= -c filter.lfs.clean= -c filter.lfs.smudge= -c filter.lfs.required=false`
when running git commands in this checkout to avoid tripping over it. The exact commit hayro's
`Cargo.toml` originally pinned (`dc23e93841d20060743e490ef49c1e0d0d22cf40`) could not be checked
out standalone for the same reason (LFS-tracked files needed for other parts of the repo); this
clone's HEAD (`af32d531fd805899d9a5c4385ceae81ce8f45b29`) is 2 commits ahead - see candidate
`001`/hayro's `003.hayro_vello_unpremultiply_wired_in` for how that confound was isolated.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command.
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real,
   plus the project's own tests where they cover the code.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
4. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a short `CORRECTION.md` in its
   folder and update `SUMMARY.md` to point at it.
