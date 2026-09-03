# iff_chunk_correct_position_lock_unlock_boundary

`lofty/src/iff/chunk.rs`, `Chunks::correct_position`'s `remaining_size > 4` lookahead-guard
boundary (the same one investigated for the plain top-level `Chunks::next()` path in
`004.iff_chunk_correct_position_padding_boundary/`), swept specifically through the
`Chunks::lock()`/`unlock()` nested-scope variant left unchecked by that candidate - flagged in
`IDEAS.md` as the most promising unchecked backlog entry this session. Note: contrary to the
backlog entry's description, `lock()`/`unlock()` is only actually used by
`lofty/src/iff/wav/read.rs` (for the RIFF `LIST`/`INFO` chunk); `lofty/src/iff/wav/tag/write.rs`
and `lofty/src/iff/aiff/tag/mod.rs` use `Chunks::new`/`.next()`/`.skip()` directly at the top
level, not `lock()`/`unlock()` - so the nested-scope path only exists in the WAV read path.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value reasoning: traced `Chunks::lock()` (computes `outer_remaining = remaining_size.saturating_sub(current_chunk_remaining_size)` once, at lock time) and `Chunks::unlock()` (calls `self.skip()` against the locked scope, then unconditionally `seek(SeekFrom::Current(self.remaining_size))` to consume whatever is left in the locked scope, *before* restoring `self.remaining_size = outer_remaining` and running a second `correct_position` at the outer level) | static read of `chunk.rs` | Two distinct `correct_position` calls are in play per `unlock()`: (1) ordinary calls made *inside* the locked scope by `Chunks::next()`/`skip()` while a RIFF-INFO sub-chunk is being parsed, where `remaining_size` denotes bytes remaining to the *container's* logical end rather than the file's end; (2) one final call made *after* restoring `remaining_size = outer_remaining`, which is structurally identical to the already-analyzed top-level case (accurate distance-to-true-EOF-or-outer-boundary), so `004`'s reasoning already covers it unchanged. For (1), the `remaining_size==4` boundary is provably self-healing: whichever branch fires (blind-padding-consume in the current code, or a hypothetical peek-then-accept/peek-then-reject under a `>=4` guard), the total number of bytes consumed by `correct_position` plus `unlock()`'s subsequent unconditional catch-up seek (`if self.remaining_size > 0 { seek(Current(self.remaining_size)) }`) always sums to exactly the `remaining_size` value that existed at `correct_position`'s entry, landing the reader at the same physical container-end position regardless of interpretation - a property `Chunks::next()`'s plain top-level path (used by `004`) doesn't have, since there's no equivalent forced catch-up seek once its outer `while` loop simply stops. Also confirmed a real subsequent sub-chunk still needs a full 8-byte header, so no real chunk can ever fit in the `remaining_size==4` boundary case regardless of which interpretation is used (same argument as `004`, applies unchanged inside the locked scope). |
| Debug-assertions build (`cargo test`, default dev profile has `debug-assertions`/`overflow-checks` on), temporary integration test at `lofty/tests/scratch_bughunt_012.rs` driving the real public `WavFile::read_from` entry point (not internal `Chunks` API) over hand-built WAV byte streams: a `LIST`/`INFO` chunk containing one `INAM` sub-chunk, with content length and trailer bytes chosen to land the locked-scope `remaining_size` exactly on 4 (no peek) vs 5 (peek attempted) at `correct_position` entry, trailer bytes crafted to *look like* a valid next-sub-chunk FourCC (`"IART"`), tested both with the `LIST` chunk at true file EOF and followed by a genuine further top-level `LIST`/`INFO` chunk; plus an exhaustive 486-combination sweep (`INAM` value length 0..=8, trailer length 0..=8 x 3 trailer byte patterns [zeros / FourCC-shaped / space-range bytes], with/without a genuine following top-level chunk), each parsed via `catch_unwind(|| WavFile::read_from(...))` and the parsed title checked against the expected `INAM` value | debug build, `cargo test -p lofty --test scratch_bughunt_012 -- --nocapture`, file deleted and `git status` confirmed clean afterward | No panics anywhere in 486 combinations. The one class of "MISMATCH" printed (54 of 486, all at `inam_len==0`) was a test-harness artifact, not a real bug: an empty `INAM` value round-trips as `title() == Some("")`, while the test's own "expected" computation used `None` for that case - every `inam_len > 0` case (all boundary/trailer/next-chunk combinations) round-tripped its title exactly, with or without a genuine following top-level chunk. |

## Status

**Not reproducible.** Covered: the full boundary interaction (`remaining_size` 4 vs. 5 at
`correct_position` entry) specifically inside the `lock()`/`unlock()` nested scope, both with the
locked container at true file EOF and followed by a real subsequent top-level chunk, across an
exhaustive 486-case sweep of content/trailer lengths and byte patterns through the real
`WavFile::read_from` public entry point - zero panics, zero incorrect tag output beyond a
self-inflicted test-script comparison artifact (documented above).

**Fully reproducible**: N/A (not Confirmed).

## Class

N/A (not reproducible)

## Root cause

Not applicable in the "confirmed bug" sense. In summary: the nested `lock()`/`unlock()` scope
does inherit the same one-byte-more-conservative-than-necessary `remaining_size > 4` guard as the
top-level path (see `004`), but `unlock()` has an additional property the top-level path lacks -
an unconditional catch-up seek (`if self.remaining_size > 0 { seek(Current(self.remaining_size)) }`)
that runs immediately after the locked scope's own parsing loop ends, regardless of how the
ambiguous trailing byte was interpreted inside that loop. This makes the boundary
self-healing specifically in the locked-scope case: no matter which way `correct_position`
decides at `remaining_size==4`, the total bytes consumed between `correct_position` and the
catch-up seek is always exactly the `remaining_size` value at entry, so the reader always lands at
the true container-end position and the outer scope resumes parsing from the correct byte
regardless. Combined with `004`'s already-established argument that no real 8-byte sub-chunk
header can ever fit within the ambiguous 4-byte window either way, this closes off the two ways
the discrepancy could plausibly matter (wrong reader position after the container, or a real
chunk silently dropped) - neither occurs.

## The fix

No fix attempted - no reproducible incorrect behavior was found to fix.

- Project test suite: not run for this candidate (no fix was made; the temporary scratch
  integration test file was deleted and `git status` verified clean - shows nothing but
  `___BUGHUNT___/` as untracked).

## What's not done / open questions

- This closes out the `004`/backlog note specifically for the `lock()`/`unlock()` nested-scope
  variant. As noted above, `lock()`/`unlock()` turned out to only actually be used by WAV's RIFF
  `LIST`/`INFO` read path (`lofty/src/iff/wav/read.rs`) - `wav/tag/write.rs` and
  `aiff/tag/mod.rs` use the plain top-level `Chunks::next()`/`.skip()` API, already covered by
  `004`'s analysis, not this nested-scope one. No further `correct_position` caller shape remains
  uninvestigated in the codebase.
- A longer, dedicated fuzzing campaign (via `fuzz-bug-repro`) against `wavfile_read_from`,
  focused on RIFF `LIST`/`INFO` chunks with truncated/ambiguous trailing sub-chunk data, was not
  run this session - the 486-case sweep here was a targeted boundary check, not a real fuzzing
  campaign (same caveat `004` already noted for the top-level path).
