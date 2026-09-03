# iff_chunk_correct_position_padding_boundary

`lofty/src/iff/chunk.rs`, `Chunks::correct_position` (~line 197-233): the padding-byte-vs-next-chunk-FourCC
lookahead guard `self.remaining_size > 4` looked like it could share the same off-by-one shape as
the confirmed `Chunks::next()` bug in `001.iff_chunk_off_by_8/` - flagged in `IDEAS.md` while
working that candidate, not previously investigated.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value reasoning (arithmetic trace of every branch of `correct_position`, comparing physical bytes consumed against `remaining_size` decrements in each branch) | static read | Every branch (chunk-size-even early return, `remaining_size==0` early return, real-padding-byte, "eating into next chunk" valid-FourCC, invalid-FourCC-after-peek) is internally self-consistent: the physical reader-position delta always matches the `remaining_size` decrement in that branch. The only anomaly found by inspection: the peek guard requires `remaining_size > 4` (i.e. `>= 5`) to attempt the 3-byte FourCC-completion read, one more byte than the `>= 4` that would be strictly sufficient (`remaining_size - 1 (already-read padding byte) >= 3`). |
| Debug-assertions build (`cargo test`, default dev profile has `debug-assertions`/`overflow-checks` on), temporary `#[cfg(test)]` unit tests added directly to `chunk.rs` driving `Chunks::<_, LittleEndian>` via its `pub(crate)` API: (a) three hand-built cases pinning `remaining_size` at correct_position entry to exactly 4, 5, and 8 with an ambiguous trailing `"DATA"`-like FourCC, observing `chunks.next()` result and the final reader position after `chunks.into_inner()`; (b) a ~3000-iteration sweep over chunk content sizes 0..12, trailing byte lengths 0..16, and several first-byte values (0, 1, `' '`, a valid-FourCC first byte), draining `Chunks::next()` to exhaustion each time | debug build, `cargo test -p lofty --lib scratch_bughunt_tests -- --nocapture`, reverted afterward | (a) Confirmed the boundary inconsistency exists and is observable at the `Chunks` level: at `remaining_size==4` the ambiguous byte is silently swallowed as a "real" padding byte (final position advances by 1, from the trailing junk's start); at `remaining_size==5` the identical-shaped ambiguous byte is correctly recognized as belonging to a look-alike next-chunk FourCC and left unconsumed (final position stays at the trailing junk's start); at `remaining_size==8` (a real, complete zero-size next chunk) the padding byte is correctly recognized as the chunk's own FourCC and the chunk is returned by `next()`. (b) No panic, no arithmetic overflow/underflow, in any of the ~3000 sweep iterations. |
| Trace through the only public-API caller path that consumes `Chunks`' final reader position (`lofty/src/iff/wav/read.rs::read_from`, which calls `chunks.into_inner()` after the parse loop and uses `data.stream_position()` as `file_length` in `properties::read_properties`) | static read of `wav/read.rs` and `wav/properties.rs` | `file_length` is only used to compute `overall_bitrate` in the branch where no `"data"` chunk's size was ever recorded (`stream_len == 0`) - an edge case a real WAV file doesn't hit. Even in that edge case, a real subsequent chunk (with its own genuine content) always needs a full 8-byte header, so `remaining_size` can only land exactly on the ambiguous `4` boundary when the trailing bytes are truly incomplete/truncated junk with nothing genuine after them - the same ambiguous input for which there is no objectively correct interpretation either way (there's no real chunk being lost or misparsed in any case tried; only a leftover-position bookkeeping difference of exactly 1 byte in a code path that doesn't affect tag/metadata output at all). |

## Status

**Not reproducible.** The `remaining_size > 4` vs. the strictly-sufficient `remaining_size >= 4`
lookahead-guard boundary is real and was empirically confirmed to produce a different internal
interpretation at exactly `remaining_size == 4` (byte treated as real padding) versus `remaining_size
== 5` (identical-shaped byte correctly recognized as the start of a next-chunk-like FourCC) - but no
input was found, by reasoning or by a ~3000-case debug-assertions sweep, where this produces a
crash, a panic, an arithmetic overflow, lost/misparsed tag or chunk data, or any other observably
wrong result through the public API. The only place the discrepancy could theoretically surface
(`WavFile::read_from`'s computed `overall_bitrate` when no `"data"` chunk exists) can only be
reached by genuinely truncated trailing data, for which the "eating into next chunk" heuristic has
no objectively correct answer either way - it's speculative best-effort recovery for malformed
input, not a case with a well-defined expected result to diverge from.

## Class

N/A (not reproducible)

## Root cause

Not applicable in the "confirmed bug" sense - see "What was tried" for the mechanism that was
investigated and ruled benign. In summary: `correct_position`'s "maybe we're eating into the next
chunk" lookahead needs 3 more physical bytes beyond the already-read padding byte (`remaining_size
>= 4` at function entry would be sufficient), but the code guards this with `remaining_size > 4`
(`>= 5`), one byte more conservative than necessary. Because a genuine follow-on chunk always needs
a full 8-byte header to be usable by `Chunks::next()` regardless, this extra unit of conservatism
never changes whether real chunk data further in the stream gets parsed or lost - it only changes,
by exactly one byte, where the reader is left positioned after iteration ends on certain
specifically-4-bytes-remaining truncated/malformed trailing input, which no caller in the codebase
observably depends on in a way that changes correctness.

## The fix

No fix attempted - no reproducible incorrect behavior was found to fix. (If a future session wants
to tidy this up purely for consistency, changing `self.remaining_size > 4` to `self.remaining_size
>= 4` would make the lookahead attempt exactly whenever it's physically safe to do so, matching the
`>= 5` case's already-correct behavior at the `== 4` boundary too - but this is a cosmetic
consistency change, not a bug fix, since no observable-output difference was found.)

- Project test suite: not run for this candidate (no fix was made; `git status` was verified clean
  after reverting the temporary investigation-only `#[cfg(test)]` module added to `chunk.rs`).

## What's not done / open questions

- The identical shared `Chunks`/`correct_position` code path is also used by AIFF parsing
  (`lofty/src/iff/aiff/read.rs`) and by the WAV/AIFF tag-writing paths
  (`lofty/src/iff/wav/tag/{mod,write}.rs`, `lofty/src/iff/aiff/tag/mod.rs`) via `lock()`/`unlock()`.
  Those weren't independently boundary-tested this session; given the mechanism is provably benign
  at the `Chunks` level itself (no wrong data, no crash, across every branch traced and swept), it's
  unlikely a different caller would expose a different outcome, but this wasn't exhaustively
  verified for every caller.
  - A longer, dedicated fuzzing campaign (via `fuzz-bug-repro`) against `wavfile_read_from`/
  `aifffile_read_from`, focused on near-EOF truncated trailing chunks, was not run this session -
  the manual sweep here was a short, targeted boundary check, not a real fuzzing campaign.
