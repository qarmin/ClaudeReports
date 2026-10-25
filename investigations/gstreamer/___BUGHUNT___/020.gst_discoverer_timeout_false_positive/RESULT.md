# gst_discoverer_timeout_false_positive

Processed the crashes accumulated by the coordinator's own long-running fuzz farm
(`___BUGHUNT___/fuzz_farm/`, `tools/run_fuzz_farm.sh` + `tools/mini_fuzz.py`, launched separately
from the main candidate loop, seeded with `tools/gen_corpus.sh`'s synthetic ffmpeg corpus). All 4
`gst-discoverer` instances independently flagged the same crash bucket (stderr hash
`a1ebdf2ba762`) within the farm's first minute of running.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Re-run the exact flagged input directly against `build-asan-fuzz/ci/fuzzing/gst-discoverer`, generous 60s wall-clock timeout, same ASAN_OPTIONS/UBSAN_OPTIONS as the farm | 4 distinct mutated FLV inputs (one per `media1`-`media4` instance, different content, same stderr bucket) | All 4 completed successfully, exit code 0, **real time exactly 10.211s each** (not a hang - consistent, bounded, identical duration across 4 different inputs) |

## Status

**Not reproducible** - as a bug. The underlying mechanism is fully understood and is not a defect:
`tools/mini_fuzz.py`'s crash-bucketing treats ANY non-zero exit code as a "crash," including its
own `subprocess.TimeoutExpired` sentinel (`rc=-9999`, bucket key literally `<timeout>` - see
`mini_fuzz.py`'s `run_target()`). `tools/run_fuzz_farm.sh` launched the `gst-discoverer` instances
with `--timeout 8` (seconds). These 4 particular malformed FLV inputs (mutated from the
`base_flv_*.flv` seeds in `___BUGHUNT___/__corpus/`) hit a code path where GStreamer's `decodebin`
can't resolve a demuxer group (`gstdecodebin2.c:3141:no_more_pads_cb: can't find group for
element`) and, before giving up and returning an error to the caller, takes ~10.2 real seconds to
do so - longer than the farm's 8s per-run budget, so `mini_fuzz.py` killed it and recorded a false
"crash".

## Fully reproducible

N/A - not a bug, so this column doesn't apply. The ~10.2s duration itself IS reliably reproducible
(all 4 inputs, identical timing) - just not a fault.

## Class

N/A (Not reproducible).

## Root cause

Not a root cause to fix - a tooling calibration issue. `GstDiscoverer`'s underlying pipeline can
legitimately take several seconds to fail cleanly on a malformed/ambiguous container (this is
normal - a real caller of the public `GstDiscoverer` API sets its own `timeout` property, whose
GStreamer-wide default is considerably more generous than 8s, precisely because probing an
unfamiliar or damaged file can be slow). `mini_fuzz.py`'s `--timeout` is a blunt "did the process
come back in time" check with no way to distinguish "hung forever" from "slow but finite" - it
does not call into `GstDiscoverer`'s own timeout semantics at all, it just wraps the whole process
in `subprocess.run(..., timeout=N)`.

Introduced by: n/a (not a bug in gstreamer's own source - it's this workspace's own harness
tuning).

## The fix

Not a code fix (nothing wrong in gstreamer itself) - a tooling fix, applied directly to this
workspace's own scripts (not reverted, since these aren't project source):

- `tools/run_fuzz_farm.sh`: bumped `gst-discoverer`'s `--timeout` from `8` to `30` (comfortably
  above the observed 10.211s worst case, with margin for a slower/more-loaded machine).
- Recorded in `~/.claude/skills/bug-hunt/projects/gstreamer.md` so a future session doesn't
  re-diagnose the same false positive from scratch, and knows to distrust a `mini_fuzz.py`
  "crash" whose `.stderr.txt` bucket key is literally `<timeout>` without re-verifying it isn't
  just slow-but-finite processing first.
- Project test suite: N/A, this candidate touches no gstreamer source at all.

## What's not done / open questions

- The other 10 fuzz-farm instances (`typefind` x6, `gst-subparse` x2, `gst-value`, `gst-tag`) found
  nothing in their run before being stopped - not re-examined here since there was nothing to
  process for them.
- Whether `GstDiscoverer`'s ~10s stall-before-giving-up behavior on this specific
  can't-find-decodebin-group condition is itself worth a closer look (e.g. is it hitting some
  fixed internal retry/backoff constant, and could a real caller be surprised by it) was NOT
  investigated - it completes and returns a clean error, it's just slower than one might expect
  for a small (1.7KB) malformed input. Not filed as a candidate since "slower than expected but
  bounded and error-handled" doesn't meet this skill's bar for a finding on its own; flagged here
  only as a possible lead if a future session wants to chase it.
