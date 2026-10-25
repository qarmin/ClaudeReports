# avidemux index-parsing integer-overflow fix (30966f12af) - looked like it reintroduced an underflow, doesn't

Noticed a cluster of five related, very recent `avidemux` bounds-checking commits while scanning
git log for the in-scope demuxers (`44383c6de7`, `30966f12af`, `0bcc6564c7`, `c8383e4848`,
`bb8fb5a9bf`) - the density suggested a recent audit/fixing pass, worth checking whether any left
a gap. Focused on `30966f12af` ("avidemux: Avoid integer overflow in bounds checks when parsing
the index") since "fixing an integer overflow bounds check" is itself an easy place to introduce
a new, different bug (e.g. an unsigned underflow) while fixing the original one.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: the commit's diff in isolation (`git show 30966f12af`) | The fix changes `if (size < 24 + bpe * (i + 1)) break;` (overflow-prone: `bpe * (i+1)` can wrap) to `if (!g_size_checked_mul(&required_size, bpe, i+1)) break; if (size - 24 < required_size) break;` in both `gst_avi_demux_parse_superindex()` and `gst_avi_demux_parse_subindex()`. The new `size - 24` is an **unsigned subtraction** - if `size < 24` this underflows to a huge value, making the very check meant to catch "not enough data" always pass instead, then falling through to `GST_READ_UINT64_LE(&data[24 + bpe*i])` on a buffer known to be smaller than 24 bytes | Looked like the fix reintroduced a different bug (unsigned underflow) while fixing the original overflow |
| Code reading: the full functions, not just the diff hunk (`subprojects/gst-plugins-good/gst/avi/gstavidemux.c`, `gst_avi_demux_parse_superindex()` ~line 1240-1324, `gst_avi_demux_parse_subindex()` ~line 1533-1613) | Both functions have their own **pre-existing, unmodified** `if (size < 24) goto too_small;` / `if (map.size < 24) goto too_small;` guard a few lines *before* the loop this commit touched (line 1261 and 1549 respectively, not part of this commit's diff) - by the time `size - 24` is evaluated inside the loop, `size >= 24` is already guaranteed, so the subtraction cannot underflow | **Rules out the bug**: the diff hunk alone (what `git show` displays) doesn't include the pre-existing guard a few lines above it, which is what makes the fix safe |
| Code reading: whether `bpe * i` (`guint16 * guint32`, ordinary C integer promotion, not the 64-bit-safe `g_size_checked_mul` path) used in the actual array index (`&data[24 + bpe * i]`) could itself overflow separately from the already-checked `required_size` | Since the loop already breaks once `required_size` (the exact, unwrapped `gsize` value of `bpe*(i+1)`) would exceed `size - 24`, and any real buffer here is a single mapped memory region (realistically far below the multi-gigabyte range needed for `bpe*i` at `guint32` width to itself wrap while still under that `required_size` cap), `bpe*i`'s ordinary-width computation stays representable in practice before the loop would have already broken | Theoretically not impossible for an implausibly large (>4GB) single index buffer, but not a practically reachable path - not pursued further |

## Status

**Not reproducible** - no bug. The fix is correct; the apparent underflow only shows up when
reading the commit's diff in isolation without the surrounding function context.

## Root cause

N/A (not a bug).

## What's not done / open questions

- The other four related avidemux commits (`44383c6de7`, `0bcc6564c7`, `c8383e4848`,
  `bb8fb5a9bf`) were read for context (git log) but not individually re-verified with the same
  depth as `30966f12af` - given this one turned out correct, the remaining four are lower-priority
  but not ruled out, and are exactly the kind of "does the diff hold up in full function context"
  check this candidate demonstrates is worth doing rather than trusting a diff in isolation.
- The theoretical `bpe * i` narrow-width overflow noted above (only reachable with an
  implausibly large single index buffer) was not fully dismissed with a citation to an actual
  practical size limit elsewhere in the RIFF/AVI chunk-size handling - worth a quick check in a
  future session if this file is revisited, though not worth pursuing further given the required
  buffer size.
