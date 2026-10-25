# gstrtph265depay.c has the same delayed-header-extension catch-all as gstrtph264depay.c

Follow-up from candidate #001 (`rtph264depay`'s `short_nal`-style drop path looked unsafe in
isolation but is covered by a general `!consumed` catch-all one call frame up) - `IDEAS.md` flagged
`gstrtph265depay.c` as not independently re-verified to have the same protection.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `grep` for the same catch-all shape in `subprojects/gst-plugins-good/gst/rtp/gstrtph265depay.c` | Found the identical `if (!consumed && rtph265depay->current_fu_type == 0) gst_rtp_base_depayload_dropped (depayload);` catch-all at line 1823-1824, in the same structural position (immediately after the main `switch (nal_unit_type)` in the packet-processing function) as `gstrtph264depay.c`'s own copy | **Rules out the bug** - H265 has the same protection as H264 |

One approach - a direct structural comparison against a pattern already fully understood from
candidate #001, not a fresh investigation needing escalation.

## Status

**Not reproducible** - no bug. Same mechanism, same protection, as candidate #001.

## Root cause

N/A (not a bug).

## What's not done / open questions

- None - this closes out the one remaining open question from candidate #001's own "what's not
  done" section.
