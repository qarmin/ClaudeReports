# id3v2 recent hardening cluster (gst-plugins-base gst-libs/gst/tag/id3v2*.c) - all verified correct

Noticed a cluster of ~7 recent id3v2 bounds/validation-hardening commits while scanning git log
for `gst-plugins-base`'s tag library (used by the in-scope, ASAN-built `id3demux` element and
`ci/fuzzing/gst-tag`'s own corpus, which includes real `id3-*` test files). The cluster includes
`955a242748` "id3v2: Don't modify const data" - textually almost identical to `ed6dcb3322`
"avidemux: Don't modify read-only mapped buffer data" (candidate #010, which turned out to
reintroduce a heap-buffer-overflow) - making this cluster a strong candidate for the same class of
regression. It wasn't - every commit checked held up.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `955a242748` "id3v2: Don't modify const data" (`id3v2_add_id3v2_frame_blob_to_taglist()`) | Moves frame-ID sanitization from mutating `frame_data` in place into a local `gchar frame_id[5]` stack buffer, and re-enables previously `#if 0`-disabled caps-setting code | Correct - no undersized-copy pattern like candidate #010's bug, this one uses a fixed-size local buffer sized to fit the fixed 4-character frame ID plus NUL, not a variable-length attacker-sized copy |
| Code reading: `7cacc0b554` "id3v2: Check for enough data when reading RVA2 tags" (`parse_relative_volume_adjustment_two()`) | Adds `if (datalen < peak_bits / 8) { ...; peak_bits = 0; }` before reading peak-value bytes. The subsequent read loop (`for (i = 0; i < GST_ROUND_UP_8(peak_bits)/8 && datalen > 0; ++i)`) has its own independent per-iteration `datalen > 0` bounds check that decrements `datalen` in lock-step with each byte actually consumed - safe regardless of the outer check's floor-vs-ceiling imprecision (`peak_bits/8` vs `GST_ROUND_UP_8(peak_bits)/8`) | Correct - real memory safety comes from the inner loop's own live bounds check, not solely the outer gate |
| Code reading: `50bc3ead97` "id3v2: Prevent undefined behaviour when shifting" (`id3v2_read_synch_uint()`) | Adds an explicit `(guint32)` cast before a left-shift of a `guint8` value that could otherwise promote to signed `int` and shift a set high bit into UB territory | Correct, minimal, matches its stated purpose exactly |
| Code reading: `6efb3cfe36` "id3v2: Add input validation and refactor id3v2_ununsync_data" (`id3v2_ununsync_data()`) | Rewrote pointer-arithmetic iteration as index-based (`in_pos`/`out_pos`); verified `out_pos` can never exceed `in_pos` (each loop iteration writes at most 1 output byte while consuming 1-2 input bytes), so the `g_malloc(*size)`-sized output buffer is never overflowed; verified the `unsync_data[in_pos+1]` lookahead read is only reached when `*size - in_pos > 1` (checked first, short-circuiting) | Correct |
| Code reading: `9c4d6d4475` "id3v2: Check valid frame sizes more" (`gst_tag_list_from_id3v2_tag()`) | Tightens `read_size < ID3V2_HDR_SIZE` to `read_size <= ID3V2_HDR_SIZE` (rejecting a header-only, zero-frame tag) and adds an equivalent strict check for the footer case before computing `frame_data_size = read_size - HDR - FOOTER` - traced both `g_assert(read_size >= ...)` calls immediately following to confirm the preceding `if` really does guarantee them (it does, with a stricter `>`, not just `>=`) | Correct |

Three more from the same cluster were also checked to close it out fully: `531ba0ebb9` "Don't
unnecessarily assert on size==0 when unsyncing data" (assertion-to-graceful-NULL conversion,
correct), `6ffd45e95d` "Improve const-correctness and use correct size type" (pure type-widening
`guint32`->`gsize`/const annotations, no behavior change), and `537161868f` "Don't try parsing
extended header if not enough data is available" (an older, already-well-established fix for a
real disclosed vulnerability, GHSL-2024-235, from 2024 - predates the 2026 cluster and was
included in the git-log scan only because it touches the same function region).

## Status

**Not reproducible** - no bug found across the five commits checked in depth. Unlike candidate
#010's avidemux cluster (where one sibling commit *did* reintroduce a bug), this id3v2 cluster's
"don't modify const data" commit specifically avoided the undersized-copy pitfall by using a
fixed-size buffer for a fixed-size field, rather than a variable-length copy sized by untrusted
data.

## Root cause

N/A (not a bug in any of the five commits examined).

## What's not done / open questions

- All commits identified via git log for this file cluster were checked (8 total) - this line of
  investigation is closed out.
- This session's `ci/fuzzing/gst-tag` short fuzz burst (candidate #006, ~10k execs) already
  exercises this exact code path with real `id3-*` corpus files and found nothing - consistent
  with, though not proof of, this candidate's code-reading conclusion.
