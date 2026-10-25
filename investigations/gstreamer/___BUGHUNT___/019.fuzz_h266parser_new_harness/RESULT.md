# Fuzz burst against a new standalone gsth266parser.c (VVC) harness

Built a standalone fuzz driver for `gsth266parser.c` this session
(`_reference/h266_harness/h266_fuzz.c` - see `README.md`'s "Existing project harnesses" section
for full details) since this file had zero prior fuzz coverage from any `ci/fuzzing/*` target,
despite having the highest density of real, historically-confirmed bugs found anywhere in this
codebase (candidate #018, 7 fixes checked, all already correctly applied). Ran a burst against it.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| New standalone harness (`_reference/h266_harness/h266_fuzz.c`), built against the ASAN+UBSAN `build-asan-fuzz/` via `gstreamer-codecparsers-1.0` pkg-config, calling `gst_h266_parser_identify_nalu()` + `gst_h266_parser_parse_nal()` (+ `gst_h266_parser_parse_sei()` for SEI NALs) directly on a whole input file | `tools/mini_fuzz.py`, corpus = 8 real VVC test vectors extracted from `subprojects/gst-plugins-bad/tests/check/libs/h266parser.c` (SPS/PPS/VPS/slice/SEI samples) via `_reference/h266_harness/extract_h266_vectors.py`, 1200s, `--timeout 5` | 50,840 execs (~42/s - much faster than the `ci/fuzzing/*` targets, since this harness has no plugin-registry-scan overhead), 0 crash buckets |

Single approach - a dumb mutation fuzzer burst is the appropriate first tool for a completely
unfuzzed file (per `HEURISTICS.md`'s tool-selection guidance), and no partial signal turned up to
escalate on.

## Status

**Not reproducible** at this exec count. 50,840 execs with zero crashes is a meaningfully larger
sample than this session's other fuzz bursts (helped by this harness's much lower per-exec
overhead - no `gst_init()`-triggered registry scan needed, since it only links the standalone
codecparsers library, not the full plugin-loading GStreamer core), but still a short, one-session
burst against a large, complex bitstream grammar (VVC/H.266) - a longer or coverage-guided
campaign would have meaningfully better odds. All 7 known-historical bugs in this exact code
region were already fixed before this session started (candidate #018), so a fresh crash from
mutation fuzzing alone was not a given.

## Root cause

N/A (no bug found this way).

## What's not done / open questions

- This harness and corpus are now reusable, standing tooling for a future session (see
  `README.md`) - a longer burst, a richer corpus (more real VVC bitstreams if available), or a
  coverage-guided fuzzer (once the AFL forkserver issue documented in
  `~/.claude/skills/bug-hunt/projects/gstreamer.md` is worked around, or via a from-scratch
  libFuzzer target using `-fsanitize=fuzzer` - now confirmed working in this container per
  `README.md`'s toolchain notes) would all be natural next steps given this file's demonstrated
  historical bug density.
- The harness only exercises `parse_nal()` (VPS/SPS/PPS/APS) and `parse_sei()` - other entry
  points in the public API (`parse_slice_hdr`, `parse_picture_hdr`, `parse_aud`, `parse_opi`,
  `parse_dci`, etc. - see `gsth266parser.h`) are reachable in principle via a real Annex-B stream
  containing those NAL types, but not all were confirmed to actually get exercised by the small
  8-file seed corpus used this session.
