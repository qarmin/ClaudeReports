# av1parser ITU-T T.35 trailing_bits() stripping (926b8a2cb0)

IDEAS.md lead: very recent bit-level bitstream parsing change (trailing-padding stripping for
AV1 T.35 metadata), flagged as a classic off-by-one/overflow-prone pattern.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `gst_av1_parse_metadata_itut_t35()` diff (926b8a2cb0), `subprojects/gst-plugins-bad/gst-libs/gst/codecparsers/gstav1parser.c` | The new backward scan (`while (remaining > 0 && payload_start[remaining-1]==0) remaining--;` then one more conditional decrement for a trailing `0x80`) only ever *decreases* `remaining` from its original `AV1_REMAINING_BYTES(br)` value and always checks `remaining > 0` before indexing `payload_start[remaining-1]` - every index touched is within `[0, original remaining)`, which was already the buffer's valid range before this commit (this commit doesn't change what memory the payload pointer/size *previously* considered valid, only shrinks the reported size by scanning backward from the already-valid end) | No injected out-of-bounds read found in the new code itself |

Only one approach used (not the full soft-cap-of-3) - the change is small (12 lines) and
self-contained enough that the bounds argument is conclusive by inspection; a fuzz/runtime check
would only be re-confirming already-clear control flow, not resolving an open question, so it
wasn't run this session (unlike candidate 001, this one didn't have a multi-layer call chain worth
tracing further).

## Status

**Not reproducible** - no bug found. This was a shallow, single-pass code review (not an
exhaustive audit of `gstav1parser.c` or its caller's own bounds-correctness), scoped specifically
to the lines this commit touched.

## Root cause

N/A (not a bug).

## What's not done / open questions

- Did not verify `AV1_REMAINING_BYTES(br)`/`GstBitReader`'s own byte-alignment bookkeeping is
  correct independent of this commit (i.e. whether `br->byte` can ever be inconsistent with the
  buffer's real size for some other, unrelated reason) - out of scope for a change that only reads
  within whatever range that macro already reports.
- Did not fuzz `ci/fuzzing/gst-codec-utils` (which exercises av1parser via H.264/H.265/AV1 sample
  files per its corpus) specifically targeting T.35 metadata payloads - the code reading was
  conclusive enough that this wasn't judged worth the setup time this session, but it's a
  reasonable quick follow-up if a future session wants empirical confirmation.
