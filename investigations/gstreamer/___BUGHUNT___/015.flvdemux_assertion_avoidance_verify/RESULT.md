# flvdemux "Avoid assertions on corrupted streams" fix (793b5b5b76) - verified complete

Discovered while scanning git log for the in-scope (built, ASAN-instrumented) `flv` plugin -
`793b5b5b76` converts several `g_assert()` calls that validated attacker-controlled tag sizes into
graceful `if (...) { ...; goto beach; }` error paths (an assertion failure aborts the whole
process - a real DoS on malformed FLV input, distinct from the memory-safety class this session's
other candidates focus on but worth the same "is the fix actually complete" scrutiny).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: the commit's diff (`gst_flv_demux_parse_tag_audio()`/`gst_flv_demux_parse_tag_video()`, `subprojects/gst-plugins-good/gst/flv/gstflvdemux.c`) | Converts four `g_assert (gst_byte_reader_get_remaining(&reader) >= (demux->tag_data_size - header_len))`-style calls (whose *own* subtraction can itself underflow if `tag_data_size < header_len`, since both are unsigned) into `if (remaining < tag_data_size - header_len) { ...; goto beach; }`, each now preceded by a matching `if (demux->tag_data_size < header_len) { ...; goto beach; }` guard added earlier in the same diff | Fix looked complete on the diff itself |
| Code reading: whether any `g_assert()` remained in the file after this commit (`grep g_assert`) | Two remain, both *added by this same commit*: `g_assert (gst_byte_reader_get_pos (&reader) >= 7)` in both the audio and video tag parsers, right before computing `header_len = pos - 7` | Needed to confirm these two are truly unreachable with `pos < 7`, not a residual DoS themselves |
| Code reading: the unconditional reads preceding each assert, from the top of `gst_flv_demux_parse_tag_audio()`/`gst_flv_demux_parse_tag_video()` | Both functions unconditionally consume at least 8 bytes (3-byte PTS + 1-byte PTS extension + 3-byte skip + 1-byte flags, for audio; the video equivalent) before reaching *any* branch that could lead to the `g_assert(pos >= 7)` line - `pos` is always >= 8 by the time that assert is evaluated, regardless of tag content, so the assert's condition is a true invariant of the code's own control flow, not a data-dependent check | **Rules out a residual bug**: both remaining asserts are unreachable-false, not a reintroduced DoS |

## Status

**Not reproducible** - no bug. The assertion-avoidance fix is complete: every attacker-data-
dependent assertion was converted to a graceful error path with a correctly-placed underflow
guard, and the two assertions the commit itself introduced check a genuine, always-true code
invariant rather than untrusted input.

## Root cause

N/A (not a bug) - see "What was tried" above for the historical bug this commit already fixed.

## What's not done / open questions

- Only `gst_flv_demux_parse_tag_audio()`/`_video()` were checked (the two functions this commit
  touched) - the rest of `gstflvdemux.c` (4879 lines) was not read in this session; git log for
  this file didn't surface another equally suspicious recent commit within the time available.
