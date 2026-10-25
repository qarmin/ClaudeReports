# gstbaseparse detect_buffers accumulation has no base-class-enforced size bound

Follow-up check on the just-fixed 65b9547519 "baseparse: Fix accumulating of detection buffers"
(IDEAS.md lead). Verified the fix itself is correct, then checked whether the surrounding
mechanism (accumulate buffers indefinitely while a subclass's `detect()` vfunc keeps returning
`GST_FLOW_NOT_NEGOTIATED`) has any bound - it doesn't, at the base class level.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `gst_base_parse_chain()` in `subprojects/gstreamer/libs/gst/base/gstbaseparse.c` (~line 3213-3290), the fixed accumulation loop and its 65b9547519 diff | Confirmed the fix is correct: `gst_buffer_copy_into()`'s `offset` parameter is validated against the *source* buffer's size (`g_return_val_if_fail (bufsize >= offset, FALSE)` in `gstbuffer.c`), and every call always *appends* to `dest`'s existing memory list regardless of `offset`/`size` - so the old per-iteration `offset += tmpsize` tracking was based on a misunderstanding of the API, and the fix's `offset=0, size=-1` (i.e. "copy the whole source buffer, appended") is the correct simplification | Confirms the fix is sound, not itself buggy |
| Code reading: `parse->priv->detect_buffers`/`detect_buffers_size` bookkeeping in the same function | No cap anywhere in the base class on `detect_buffers_size` or the list length - a subclass whose `detect()` keeps returning `GST_FLOW_NOT_NEGOTIATED` causes buffers to accumulate without bound (`g_list_append` + `detect_buffers_size += gst_buffer_get_size(buffer)`, checked at 3273-3276) | Looked like a potential unbounded-memory DoS at the base-class level |
| Code reading: the only current implementor, `gst_vc1_parse_detect()` in `subprojects/gst-plugins-bad/gst/videoparsers/gstvc1parse.c` (~line 636-720) | It self-bounds: `if (gst_buffer_get_size (buffer) <= 128) { ...; return GST_FLOW_NOT_NEGOTIATED; }` - once the accumulated buffer exceeds 128 bytes, `vc1parse` unconditionally commits to a heuristic guess or returns `GST_FLOW_ERROR`, so it never keeps requesting more data past ~128 bytes | **Not exploitable today**: the base class's lack of a bound is real, but the sole subclass that could trigger it already imposes its own tight bound, so there's no accessible attacker-controlled unbounded-growth path in the current codebase |

## Status

**Not reproducible** (with the current codebase's only `detect()` implementor). The base class's
lack of an enforced cap on `detect_buffers_size` is a real absence of defense-in-depth - a
*hypothetical future* subclass that implements `detect()` without imposing its own bound (unlike
`vc1parse`) would reintroduce an unbounded-memory-growth DoS via a stream that never resolves to a
detectable format - but no such subclass exists in this codebase today, so there is nothing to
build a genuine end-to-end repro against.

## Root cause

N/A (not a bug in current code) - see "What was tried" above.

## What's not done / open questions

- If GStreamer ever gains a second `detect()`-implementing element that doesn't self-bound like
  `vc1parse` does, this becomes a real, easily-triggered DoS (feed an endless stream of
  never-quite-detectable data). Worth a defense-in-depth patch at the base-class level
  (`gst_base_parse_chain()`, cap `detect_buffers_size` against a `GST_BASE_PARSE_MAX_...`-style
  limit and return `GST_FLOW_ERROR` past it) regardless, but that's a design suggestion, not a
  confirmed bug - not attempted as a "fix" here since there's no reproduction to verify it against.
