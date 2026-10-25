# Chunked + flush_pixels decode versus single-shot decode

`flush_pixels` and incremental feeding go through different pipeline code than a one-shot decode; the final frame must be bit-identical. The repo has a sequential-vs-parallel differential target but none for chunked-vs-whole.

Base commit 7f539c8 (jxl-rs main); the main tree was not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| cargo-fuzz harness `chunkdiff`: fuzz_decode(default) vs fuzz_decode(progressive chunk sizes, flush_intermediate), bit-exact compare | 8 forks, 1513 s, ~614k execs, seeded with the 91 small test files | 0 mismatches, and chunked decode never failed where the whole decode succeeded |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

Bit-exact agreement on every input that decoded.

## What's not done / open questions

- Same fuzzing-depth caveat as candidate 003.
