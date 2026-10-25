# Allocation failure must be an Err, not an abort

The decoder is used with `sample_limit: None` by default and the fuzz targets always set a limit, so an infallible allocation (`vec![..]`, `Vec::with_capacity`) driven by header values would only show up as an abort.

Base commit 7f539c8; main tree not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| cargo-fuzz `nolimit` (sanitizer none, `sample_limit: None`) under `ulimit -v 3500000`, 8 forks, 1500 s, ~1.1M execs, max_len 30000, seeded with 59 small test files | `harnesses/fuzz_formats` target `nolimit` | 0 crashes, 0 OOM reports (an abort from a failed allocation would be a SIGABRT crash) |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

No input within the mutation reach made the decoder abort on allocation failure.

## What's not done / open questions

- Inputs rarely reach a valid frame header with huge dimensions; see IDEAS.md for the crafted-header lead.
