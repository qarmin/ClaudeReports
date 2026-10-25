# avidemux vprp divisor fix + FUJIFILM strd fixes - both already correctly applied

Last two items from the avidemux bounds-checking commit cluster (see candidates #008-#010).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `0bcc6564c7` "Use correct divisor for calculating available number of vprp field infos" - `gst_avi_demux_riff_parse_vprp()` (~line 1796-1846) | Old code computed `k = (size - offset) / vprp->fields` (dividing available bytes by the attacker-controlled *field count* instead of by one entry's *size*) before comparing `vprp->fields > k` to clamp it - a nonsense divisor that could leave `vprp->fields` unclamped even when the real buffer (allocated as exactly `size` bytes via `g_malloc(size)`, not padded) doesn't actually hold that many `field_info` entries, risking an OOB read when later code iterates `vprp->field_info[0..fields-1]`. Current `HEAD` already has the fix (`k = (size - offset) / sizeof(vprp->field_info[0])`), and a separate, unrelated, pre-existing `if (vprp->fields == 0) return FALSE;` check (line 1826) means the *old* code was never actually at risk of a division-by-zero either way - the bug was purely the wrong-but-nonzero divisor producing an incorrect, too-permissive `k` | Already correctly fixed in `HEAD` |
| Code reading: `bb8fb5a9bf` "Make sure enough data is available when parsing FUJIFILM strd" - `gst_avi_demux_parse_strd()` (~line 3835-3875) | Old code had `while (ptr[sub_size] && sub_size < left) sub_size++;` in three places - evaluating `ptr[sub_size]` *before* checking `sub_size < left` in a `&&` short-circuit is backwards, reading one byte out of bounds whenever `sub_size` had already reached `left`. Also had the exact same "date colon-fixup at fixed offsets 4/7 with no length check" pattern candidate #010 found and fixed in `parse_ncdt()`'s sibling function. Current `HEAD` already reorders all three loop conditions to `sub_size < left && ptr[sub_size]` (short-circuiting safely) and already added `if (sub_size >= 8) { ...colon fixup...; parse_tag_value(...); }` around the date-string handling | Already correctly fixed in `HEAD` - and independently confirms candidate #010's `sub_size >= 8` fix is the same shape the original author used for the identical bug class in a sibling function |

## Status

**Not reproducible** for both - no bug, both already fixed in current `HEAD`.

## Root cause

N/A (not bugs in current code) - both were real, now-fixed issues; see each fix's own commit for
the historical window during which they were live (not investigated further since both are
already merged).

## What's not done / open questions

- This closes out every item from the avidemux bounds-checking cluster that was flagged in
  `IDEAS.md` this session (`30966f12af`, `44383c6de7` - candidate #008; `ed6dcb3322` - candidate
  #010, the one that turned out to still be broken; `0bcc6564c7`, `bb8fb5a9bf` - this candidate).
  No further avidemux leads remain queued from this specific commit series.
- `bb8fb5a9bf`'s independent confirmation of the `sub_size >= 8` pattern is reassuring but was not
  itself treated as a substitute for candidate #010's own runtime verification - both were
  verified separately by design (code reading here, real ASAN repro there).
