# Short mutation-fuzz burst against typefind/gst-subparse/gst-tag/gst-value

IDEAS.md leads: the four shallower existing `ci/fuzzing/*` oss-fuzz targets (excluding
`gst-discoverer`, covered separately in `007.fuzz_gst-discoverer/`). Ran `tools/mini_fuzz.py`
(this workspace's own dependency-free mutation fuzzer, built this session - see `../README.md`)
against the ASAN+UBSAN build for ~20 minutes each.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `tools/mini_fuzz.py` vs. `typefind` | ASAN+UBSAN build, corpus = 29 real media files (copied from `gst-discoverer`'s own corpus list, since typefind has no dedicated one), 1200s, `--timeout 5` | 10,396 execs (~9/s), 0 crash buckets |
| `tools/mini_fuzz.py` vs. `gst-subparse` | ASAN+UBSAN build, corpus = 2 real `.srt`/`.vtt`/`.sub` files, 1200s, `--timeout 5` | 29,725 execs (~25/s), 0 crash buckets |
| `tools/mini_fuzz.py` vs. `gst-tag` | ASAN+UBSAN build, corpus = 2 real ID3/vorbis-tag files, 1200s, `--timeout 5` | 10,629 execs (~9/s), 0 crash buckets |
| `tools/mini_fuzz.py` vs. `gst-value` | ASAN+UBSAN build, corpus = 11 real `GstValue` serialization strings (`ci/fuzzing/seeds/gst-value/`), 1200s, `--timeout 5` | 32,070 execs (~27/s), 0 crash buckets |

Soft cap of 3 approaches doesn't really apply here in the usual sense (this is breadth across
targets, not depth on one suspect) - each target got exactly one fuzzing approach, which is
appropriate: a dumb mutation fuzzer with no coverage feedback has diminishing returns per target
past a certain point, and none of the four showed even a partial/flaky signal worth escalating on.

## Status

**Not reproducible** for all four targets, at this exec count. Execs/sec was low (9-27/s -
`mini_fuzz.py`'s per-run `subprocess.run()` + temp-file-write overhead dominates over the
binaries' own ~10-15ms execution time once the registry cache is warm) - a real coverage-guided
campaign (afl-fuzz, if the forkserver incompatibility documented in
`~/.claude/skills/bug-hunt/projects/gstreamer.md` gets fixed, or libFuzzer proper) would cover far
more state per unit time than this dumb mutation approach. These four targets have also been
part of Google's oss-fuzz continuous campaign for years, so a fresh crash from ~10-30k naive
mutation execs in one session was always a long shot - this burst was worth running (cheap, ran in
the background while other candidates were investigated by code reading) but code reading turned
out to be the higher-yield activity this session (see candidate #005).

## Root cause

N/A (no bug found this way).

## What's not done / open questions

- A longer burst (hours, not ~20 minutes) or a real coverage-guided fuzzer would have materially
  better odds - see `~/.claude/skills/bug-hunt/projects/gstreamer.md` for the AFL forkserver issue
  that needs solving first, or consider `fuzz-bug-repro` (a separate skill) for a dedicated,
  longer campaign if a future session has the time budget for one.
- Seed corpora were small (2-29 files) and hand-picked from existing test fixtures - a richer
  corpus (e.g. every file under `subprojects/*/tests/files/`) was not assembled this session for
  time reasons.
