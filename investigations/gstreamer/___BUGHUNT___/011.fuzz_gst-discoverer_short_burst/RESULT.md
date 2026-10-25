# Short mutation-fuzz burst against gst-discoverer

IDEAS.md lead: `ci/fuzzing/gst-discoverer.c`, the deepest existing oss-fuzz target (builds a real
`GstDiscoverer`/decodebin pipeline). First attempt used `mini_fuzz.py`'s default `--timeout 5`,
which is shorter than the harness's own internal 10s discovery timeout and produced a false
"crash" (bucket key `<timeout>`, exit -9999) - documented as an environment gotcha in
`../README.md` and `~/.claude/skills/bug-hunt/projects/gstreamer.md`. Restarted with `--timeout 15`
for a real run.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `tools/mini_fuzz.py` vs. `gst-discoverer`, `--timeout 5` | ASAN+UBSAN build, corpus = 29 real media files (ogg/mkv/mp4/webm/ts/wav/flv/mp3), 1200s | False-positive-only: 1 `<timeout>` bucket, confirmed to be the harness's own 10s internal timeout exceeding the fuzzer's 5s kill timeout, not a real hang - deleted, not a finding |
| `tools/mini_fuzz.py` vs. `gst-discoverer`, `--timeout 15` (corrected) | Same build/corpus, 1500s | 8,698 execs (~6/s - the slowest of all `ci/fuzzing/*` targets tried this session, expected given it builds a full pipeline per input), 0 crash buckets |

## Status

**Not reproducible** at this exec count (8,698, ~6/s over 25 minutes). Same caveat as candidate
#006: a dumb mutation fuzzer with no coverage feedback, against a target this deep, needs
significantly more wall-clock time or a real coverage-guided fuzzer to have good odds - this was a
reasonable-cost spot check, not an exhaustive campaign. A separate, larger, longer-running fuzzing
operation (`___BUGHUNT___/fuzz_farm/`, launched independently by the user/coordinator, up to 6h,
using a richer ffmpeg-generated corpus) is running in parallel and covers this and other targets
far more thoroughly than this candidate's own short burst - see that directory's own results
separately when reviewed.

## Root cause

N/A (no bug found this way; see candidate #010 for what code reading against the *same general
codebase area* - `gst-plugins-good` demuxers - found instead this session).

## What's not done / open questions

- See candidate #006's identical note on AFL forkserver incompatibility
  (`~/.claude/skills/bug-hunt/projects/gstreamer.md`) - the same applies here, `gst-discoverer` was
  the actual target used to originally diagnose that issue.
- The parallel `fuzz_farm/` results were deliberately left unanalyzed per the user's own request
  (pure crash accumulation for later review) - not incorporated into this workspace's `SUMMARY.md`
  by this session; a future session picking this project back up should check
  `___BUGHUNT___/fuzz_farm/crashes_*/` for anything beyond the one already-known `<timeout>`
  false-positive (confirmed identical hash `a1ebdf2ba762` to this candidate's own first attempt)
  before assuming it's all triaged.
