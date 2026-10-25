# rtph264depay delayed RTP header extension leak via short/zero-size STAP-A NALU

`gstrtph264depay.c`'s `gst_rtp_h264_depay_handle_nal()` has a `short_nal:` error path (triggered
by a zero- or near-zero-size aggregated NALU, reachable via an attacker-crafted STAP-A packet with
a NALU-size field of 0) that drops the NAL without calling `gst_rtp_base_depayload_flush()` or
`gst_rtp_base_depayload_dropped()` - the same bug class as three just-preceding commits
(bb387b3c7b, 9e627c3d88, a7e6f79e3b, all part of MR !12389) that fixed RTP header extensions
leaking into the wrong output buffer. Investigated as IDEAS.md's RTP-depayloader lead.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `gst_rtp_h264_depay_handle_nal()`'s `short_nal:` label (map.size < 5) | Confirmed it returns `FALSE` (not consumed) without calling `gst_rtp_base_depayload_flush()`/`_dropped()`, unlike the sibling "waiting for keyframe" drop path a few lines above it (which does call `flush()`) | Looked like a real gap at the function level |
| Code reading: reachability of `short_nal` from a STAP-A (nal_unit_type=24) packet | A STAP-A NALU-size field of `0x0000` produces a 0-byte NAL payload -> `outsize = 0 + sizeof(sync_bytes) = 4` -> `map.size (4) < 5` -> `short_nal`. Confirmed reachable with a single, minimal, spec-shaped STAP-A packet (no need for boundary/clamping edge cases - a literal size-0 field triggers it directly) | Confirmed reachable |
| Code reading: the calling context, `gst_rtp_h264_depay_process()` | Found a **general catch-all** immediately after the `switch (nal_unit_type)` block (not part of the 3 commits above - already present in this exact form in current `HEAD`): `if (!consumed && rtph264depay->current_fu_type == 0) gst_rtp_base_depayload_dropped (depayload);`. For a STAP-A packet whose only aggregated NALU is the short/zero-size one, `consumed` (`|=` accumulated across the STAP-A loop) stays `FALSE`, so this catch-all fires and correctly discards the packet's own queued header-extension entry via `gst_rtp_base_depayload_dropped()`. Traced every other path that could make `handle_nal` return `TRUE` despite a real per-sub-NAL drop (multi-NALU STAP-A packets where only some sub-NALs are short) and found those cases are actually correct: the packet's single header-extension entry is legitimately consumed by whichever sibling sub-NAL from the *same* RTP packet does get pushed - not a leak into an unrelated *later* packet's output. | **Rules out the bug**: the `short_nal` gap at the `handle_nal()` function level is compensated for by this coarser, packet-level catch-all in the caller. |

Soft cap of 3 approaches (per SKILL.md) reached without a live signal - all three converged on the
same conclusion (no bug), so no further escalation (no runtime fuzzing/repro attempted - the code
reading was conclusive enough on its own that a repro would only reconfirm already-clear control
flow, not resolve an open question).

## Status

**Not reproducible.** The specific mechanism suspected (a `short_nal` drop leaking header
extensions into the next output buffer) is real at the level of `gst_rtp_h264_depay_handle_nal()`
in isolation, but is masked by a general "nothing from this RTP packet was consumed" catch-all one
call frame up, in `gst_rtp_h264_depay_process()`, which already existed in current `HEAD` (not
part of the 3 commits that prompted this investigation - it looks like an earlier, more general
fix for the same underlying bug class, landed before those 3, or in the same MR but a hunk not
shown by `git show --stat` on just those 3 commits). No runtime repro was built since the code
reading traced the exact boolean logic (`consumed` accumulation across the STAP-A loop, the
`current_fu_type == 0` guard) to a clear, unambiguous "already handled" conclusion.

## Root cause

N/A (not a bug) - see "What was tried" above for why the suspected gap doesn't manifest.

## What's not done / open questions

- **`gstrtph265depay.c` was not independently checked** - it very likely has the same
  `short_nal`-style path and the same (or a differently-shaped) catch-all, given the 3 prompting
  commits touch both files identically. Given the H264 file's catch-all already rules out the bug
  class, H265 was deprioritized rather than re-investigated from scratch this session - a future
  session should verify H265 has an equivalent catch-all (or confirm the bug class genuinely
  doesn't apply there either) before assuming parity.
- The exact commit that introduced the `if (!consumed && current_fu_type == 0) dropped();`
  catch-all was not tracked down (not necessary once the current-`HEAD` behavior was confirmed
  correct - "Introduced by" doesn't apply to a ruled-out candidate).
- No fuzzing/dynamic testing was used for this candidate - purely code reading. If a future session
  wants to double-check this conclusion empirically, a `GstHarness`-based test (see
  `subprojects/gst-plugins-good/tests/check/elements/rtph264.c`'s own `GstHarness` usage and
  `subprojects/gst-plugins-base/tests/check/libs/rtpdummyhdrextimpl.c`'s reusable dummy header
  extension GType) would be the right tool - register the dummy extension via the `add-extension`
  signal, push a STAP-A packet with a zero-size NALU carrying one marker byte, then a second,
  unrelated NAL, and assert the second output buffer's extension read does NOT see the first
  packet's marker byte.
