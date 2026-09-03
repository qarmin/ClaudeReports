# chunk_file_junk_padding_and_fallback_arithmetic

Follow-up to candidates `015`/`016`, which confirmed and fixed a wrong-sign `u64` subtraction in
`lofty/src/id3/v2/write/chunk_file.rs`'s `write_to_chunk_file`, but only in the "overwrite ID3v2
tag at end of file" fast path. This candidate independently checks the two sibling subtraction
sites in the same function that were only manually read/traced at the time, per the `IDEAS.md`
follow-up: the WAV `JUNK`-padding path (`existing_tag_len - u64::from(IFF_CHUNK_HEADER_SIZE)` /
`existing_tag_len - tag_chunk_size`, ~lines 135-136) and the shared full-file-rewrite fallback
(`file_context.stream_length -= existing_id3_tag.end - existing_id3_tag.start`, ~line 169).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual invariant tracing | Read `Chunks::next`/`Chunks::skip`/`Chunks::correct_position` in `lofty/src/iff/chunk.rs` and `find_existing_id3v2_tag` in `chunk_file.rs` | `existing_tag_len` (`existing_id3_tag.end - existing_id3_tag.start`, used at both suspect sites) is `chunks.stream_position()` after `skip()` minus `chunk.start()` before the header was read. `Chunks::next` only ever returns a `Chunk` after successfully consuming a full 8-byte (`IFF_CHUNK_HEADER_SIZE`) header, so the minimum possible value is exactly 8 (a real chunk with `size == 0`) - `existing_tag_len - IFF_CHUNK_HEADER_SIZE` (line 135) can never underflow. Line 136's `remainder = existing_tag_len - tag_chunk_size` only runs inside the `if (existing_tag_len - 8) >= tag_chunk_size` guard, so `existing_tag_len > tag_chunk_size` is already established. For line 169, `file_context.stream_length` is set once to `actual_stream_size` from the first (RIFF/FORM) chunk header and never modified before this subtraction; `existing_id3_tag`'s range was found by a second `Chunks` reader explicitly bounded to `stream_length - IFF_CHUNK_HEADER_SIZE` remaining bytes, so the tag's length is provably `<= stream_length`, and this subtraction can't underflow either. |
| Real end-to-end boundary-value sweep (`___BUGHUNT___/_reference/scratch_017`), release profile with `overflow-checks = true`, driven entirely through the real `WavFile`/`AiffFile::read_from` -> `set_id3v2` -> `save_to` public API (plus one hand-crafted, maximally-adversarial on-disk WAV file fed to the same real entry point) | Built WAV/AIFF files with a real ID3v2 tag placed as a *non-terminal* chunk (a real trailing chunk follows it, forcing every subsequent write through the `JUNK`-padding path or the full-rewrite fallback instead of the already-covered end-of-file fast path); swept 5 magnitudes of `existing_tag_len` (initial tag sizes 0/50/500/2000/8000 chars) x new-tag sizes 0..=4000 (step 1) x 3 write configurations (WAV `preferred_padding=Some(8)`, WAV `preferred_padding=None`, AIFF) = 60,015 real `save_to` calls, plus 402 more against a hand-crafted WAV with a bare zero-length `ID3 ` chunk (`existing_tag_len == 8` exactly, the tightest possible boundary for line 135's guard) | **0 panics, 0 silent-corruption detections** (checked via the RIFF/FORM header's declared size vs. the actual physical output size after every write) across all 60,417 real end-to-end writes. `RUST_LOG=trace` confirmed the `JUNK`-padding "large enough to overwrite" branch (line 137's log message) was genuinely taken 6,509 times across the sweep, not just attempted - so both suspect sites were actually exercised, not skipped over. |

## Status

**Not reproducible.**

Covered: both suspect arithmetic sites, via manual invariant proof (an unconditional, structural
guarantee independent of any specific input, since it follows from how `Chunks::next`/`skip`
consume bytes) plus a real, densely-swept boundary-value exercise of both branches (the
`JUNK`-padding "large enough" branch and the full-rewrite fallback) through the real public
write API, including the single tightest edge case (`existing_tag_len == IFF_CHUNK_HEADER_SIZE`
exactly) via a hand-crafted adversarial file. No signal (no panic, no wrong-sign/overflow symptom,
no silent corruption) was seen at any point, so a third tool (e.g. a persistent fuzz harness) was
not pursued, per `SKILL.md`'s "3 clean negatives means move on" guidance - here 2 independent
approaches (proof + a 60k+-case real exercise) already agree with no partial signal to chase.

## Class

N/A (not reproduced).

## Root cause

N/A - no bug found. Both sites are safe, but for different structural reasons than the fast-path
bug fixed in `015`/`016`:

- The `JUNK`-padding path's `existing_tag_len - IFF_CHUNK_HEADER_SIZE` is safe because
  `existing_tag_len` is *always* at least `IFF_CHUNK_HEADER_SIZE` by construction of how
  `Chunks::next` parses a chunk (it cannot return a `Chunk` without having first consumed a full
  8-byte header) - this holds for *any* on-disk chunk, not just ones lofty itself wrote.
- Its `existing_tag_len - tag_chunk_size` (the padding "remainder") is safe because it only
  executes after an `if` that already establishes `existing_tag_len > tag_chunk_size` (in fact
  `existing_tag_len - 8 >= tag_chunk_size`, an even stronger guarantee).
- The full-rewrite fallback's `stream_length -= existing_tag_len` is safe because
  `file_context.stream_length` is fixed once (from the outer RIFF/FORM chunk's declared size) and
  `existing_id3_tag`'s range was discovered by a chunk reader explicitly bounded to stay within
  that same `stream_length`, so the existing tag's length can never exceed it.

Unlike the fast path's bug (a genuinely wrong operator, `-=` where `+=` was needed, that no amount
of guarding would have caught), these two sites don't have an operator-sign problem at all - the
subtractions are the correct operation, and the values involved are provably ordered correctly by
the surrounding control flow/invariants, not merely "usually" ordered correctly.

## The fix

N/A - no bug found to fix.

- Project test suite: not run (no code change was made; nothing to verify).

## What's not done / open questions

- The boundary sweep used `comment_len` (ID3 comment-frame length) as the size-varying knob for
  the *new* tag on every iteration; it did not independently vary other frame types/counts that
  could produce the same `tag_chunk_size` via a different code path through `create_tag`/
  `frame::create_items` - considered low-risk here since `tag_chunk_size` in `chunk_file.rs` only
  ever consumes the final serialized byte length (`tag.len()`), not any per-frame structure.
- Did not fuzz the on-disk chunk *padding byte* logic (`Chunks::correct_position`'s
  odd-length-chunk handling, already covered separately by candidates `004`/`012`) in combination
  with these two write-path sites specifically - e.g. an existing `ID3 ` chunk whose declared size
  is odd and whose padding byte is itself adversarial. Considered low-risk since `existing_tag_len`
  is derived from `chunks.stream_position()` after `skip()` has already resolved any padding
  ambiguity, not from the chunk's raw declared `size` field.
- This closes out the `IDEAS.md` follow-up left by candidate `016`; no further un-investigated
  subtraction site remains in `chunk_file.rs` (all of: the fast path confirmed in `015`/`016`, the
  `JUNK`-padding path, and the full-rewrite fallback, are now accounted for).
