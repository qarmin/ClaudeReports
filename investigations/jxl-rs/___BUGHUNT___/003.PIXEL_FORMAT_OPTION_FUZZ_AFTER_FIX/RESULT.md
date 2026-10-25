# API pixel-format / decoder-option space after the 001 fix

After candidate 001 the same harness kept running: the repo's own fuzz targets only ever request the default f32 format, so every combination of colour type, sample type, bit depth, endianness, per-extra-channel format, orientation / spot-colour / coalescing / premultiply / high-precision / skip-preview options, Main10, aux-box requests, chunked input and mid-stream `flush_pixels` was unexplored.

Base commit 7f539c8 (jxl-rs main); the main tree was not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| cargo-fuzz harness `formats` (first 9 bytes choose pixel format + options, rest is the codestream) | built against the 001-patched tree, 8 forks, 2400 s, ~353k execs, seeded with 444 inputs (91 test files x ~5 random prefixes) | 0 crashes (a continued 1500 s run from the saved corpus: ~191k more execs, 0 crashes) |
| cargo-fuzz harness `chunked` (adds Main10, aux boxes + close_input, chunked feeding, flush_pixels on NeedMoreInput) | 6 forks, 1800 s, ~544k execs; a second 1500 s run with 7 forks continued from the saved corpus | 0 crashes; the continued run (1500 s, ~460k further execs) also 0 |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

Nothing besides the 001 crash was found. All 24+ crash inputs of the first `formats` run were the same panic and pass with the 001 patch.

## What's not done / open questions

- Fuzzing is shallow: ~10 CPU-hours at most, mutation based with seeds that are small test files, so deep VarDCT/modular states of large images are rarely reached. Harness sources are in `harnesses/fuzz_formats`.
