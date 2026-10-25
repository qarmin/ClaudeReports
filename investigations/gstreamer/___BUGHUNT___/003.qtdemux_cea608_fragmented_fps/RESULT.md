# qtdemux CEA-608/fragmented-mp4 framerate handling (784ba21c54 + 6212f916b2)

IDEAS.md lead: two related, very recent qtdemux commits touching fragmented-MP4 framerate
handling and CEA-608 caption splitting. Checked whether the pair fully closes the gap between
them or leaves a division-by-zero window.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: 784ba21c54 "Don't guess framerate for fragmented mp4" | For any fragmented stream, `gst_qtdemux_guess_framerate()` now unconditionally sets `fps_n=0, fps_d=1` instead of computing a (fragment-unstable) real framerate | Confirms fragmented CEA-608 streams now always have `fps_n == 0` |
| Code reading: `gst_qtdemux_split_and_push_buffer()` (~line 7018-7145), both before and after 6212f916b2 | The CEA-608 caption-splitting branch divides by `CUR_STREAM(stream)->fps_n` twice (`gst_util_uint64_scale(..., fps_d, fps_n)` at lines 7115 and 7118, computing `out_pts`/`out_dur`). Before 6212f916b2 the branch's guard only checked `fps_d` truthy, not `fps_n` - combined with 784ba21c54 forcing `fps_n=0` on every fragmented stream, this was a **guaranteed division-by-zero on any fragmented MP4 with an embedded CEA-608 (`c608`/`clcp`) track that needs splitting** (`n_field1 > 1 \|\| n_field2 > 1`, i.e. more than one caption pair per sample) | Would have been a real, easily-triggered crash (a completely spec-legal fragmented MP4 with embedded closed captions, not even a malicious file) |
| Code reading: current `HEAD` (6212f916b2 already applied) | The guard now also checks `CUR_STREAM (stream)->fps_n` truthy before entering the branch at all - with `fps_n == 0` for every fragmented stream, the branch is now skipped entirely for fragmented content, falling through to the plain `gst_qtdemux_push_buffer()` else-branch (no splitting, no division) | **Already fixed** in current `HEAD` - confirmed no remaining call site divides by `fps_n`/`fps_d` without the same guard (only the two divisions inside this one now-guarded `if` block reference them) |

## Status

**Not reproducible** against current `HEAD` - the division-by-zero this pair of commits could have
left behind (had only one of the two landed, or had 6212f916b2 not checked `fps_n` specifically)
is fully closed. This is a good example of "checked whether a very recent fix is complete" from
`SKILL.md`'s discovery heuristics turning up a real historical near-miss that's already resolved,
not a live bug.

## Root cause

N/A (not a bug in current code). Historical: 784ba21c54 (landed 2026-03-17) made `fps_n == 0` the
normal case for all fragmented streams; had 6212f916b2 (landed 2026-09-10, ~6 months later) not
also checked `fps_n`, every fragmented MP4 with a multi-caption-pair CEA-608 sample would have hit
`gst_util_uint64_scale(x, fps_d, 0)` - worth noting this as a real gap that *did* exist in `HEAD`
between those two dates, in case anyone needs to know whether a specific historical build/release
in that window is affected, but not something to act on now since both commits are already merged.

## What's not done / open questions

- Did not build a runtime repro (a `.mp4` fragment with an embedded multi-pair CEA-608 track) since
  the code reading conclusively shows the guard is already correct in `HEAD` - a repro would only
  demonstrate a bug in a *specific historical commit range*, not in the code this session is meant
  to be checking (current `HEAD`).
- Did not check whether `gst_util_uint64_scale()` itself has any defined behavior for a zero
  denominator (e.g. an assertion vs. silently returning `G_MAXUINT64`) - moot given the guard
  already prevents reaching it, but worth knowing for HEURISTICS.md if a similar pattern turns up
  elsewhere in this codebase (grep for `gst_util_uint64_scale.*fps_n` or similar ungated divisions
  would be a cheap follow-up static check for a future session).
