# iff_chunk_off_by_8

`lofty/src/iff/chunk.rs`, `Chunks::next()` (~line 104): the bounds check `if u64::from(size) >
self.remaining_size` ran before `self.remaining_size -= IFF_CHUNK_HEADER_SIZE` accounted for the 8
header bytes (FourCC + size) already consumed off the reader, so a chunk whose declared content
size overshoots the truly-remaining bytes by up to 8 was wrongly accepted as valid. This candidate
came from a prior static-review pass in `IDEAS.md`; this session built and ran a real standalone
repro against the public API to confirm it end-to-end.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + standalone repro via public API (`WavFile::read_from`) | release build, in-memory `Cursor<Vec<u8>>` WAV: valid `fmt ` + valid `data` chunks followed by a trailing `ID3 ` chunk header declaring 8 content bytes with 0 physically present | Reproduced: `Err(FileParseError)` on unpatched code, `Ok(WavFile)` on patched code |
| Same repro, second boundary value in the class (declared size 5 instead of 8, both within the 1..=8 off-by window) | release build, same harness | Reproduced identically (Err before / Ok after), ruling out the first value being a coincidence |
| Repeat-run check | `RUNS=5`, release build, same input | 5/5 - fully deterministic, no build-flag/layout/seed dependence |

## Status

**Confirmed**. Deterministic 5/5 release-build repeat runs on the exact triggering input, plus a
second boundary value (declared overshoot of 5 instead of 8) in the same off-by-window class,
also reproduced deterministically before the fix and resolved after it.

**Fully reproducible**: **Yes**. The repro calls `WavFile::read_from` (the real, public
`AudioFile::read_from` entry point) directly on a malformed byte buffer - a malformed file handed
to a parser's public entry point *is* the genuine external input, with no second participant to
fake (per `CONFIRMATION.md` section 11's explicit "single-process, single-participant repro
remains completely fine" case).

## Class

logic

## Root cause

`Chunks::next()` reads the 8-byte chunk header (4-byte FourCC + 4-byte little/big-endian `size`)
directly off the underlying reader, then validates the declared `size` against
`self.remaining_size` *before* `self.remaining_size` has been decremented by
`IFF_CHUNK_HEADER_SIZE` (8) to account for those just-consumed header bytes. At the moment of the
check, `self.remaining_size` still counts the header bytes that are no longer actually available
as chunk content, so the check effectively allows `size` to be up to 8 bytes larger than the
content bytes truly remaining in the stream.

When such an oversized-but-accepted chunk's content is later actually read (e.g. via
`Chunk::content()` -> `read_exact`), the read hits genuine physical EOF and returns a hard
`std::io::ErrorKind::UnexpectedEof`, which propagates via `?` all the way out of
`WavFile::read_from`/`AiffFile::read_from` as `Err(FileParseError)` - even though the caller
requested `ParsingMode::BestAttempt` (the default), whose entire purpose is to tolerate exactly
this kind of truncated/corrupted trailing data and keep whatever was already parsed successfully
(the `fmt`/`data` chunks in the repro).

This is shared code (`lofty/src/iff/chunk.rs`) used identically by `iff/wav/read.rs` and
`iff/aiff/read.rs`, so the same defect applies to AIFF parsing as well; it was not independently
re-verified with an AIFF-specific repro this session (see "What's not done" below).

## Introduced by

b64f9722 "IFF: Support invalid chunk padding" (2026-02-27) - `git blame` on the current check-before-decrement order (`chunk.rs` lines 104/119) attributes both the `size > self.remaining_size` check and the later `self.remaining_size -= IFF_CHUNK_HEADER_SIZE` to this commit, which restructured the function into its current shape. An earlier commit, 8a70a773 "AIFF: Stop relying on file's provided size (Fix OOM)" (2022-07-10), introduced `remaining_size` itself but decremented it inside a separate `read_header` step before the size check ran, a different (correct-order) structure that b64f9722 later replaced.

## The fix

Move the `self.remaining_size -= u64::from(IFF_CHUNK_HEADER_SIZE)` subtraction to run *before* the
`size > self.remaining_size` comparison, instead of after it, so the check compares the declared
content size against the bytes truly available for content (the earlier `if self.remaining_size <
u64::from(IFF_CHUNK_HEADER_SIZE) { return Ok(None) }` guard a few lines above already guarantees
this subtraction can't underflow).

```diff
--- a/lofty/src/iff/chunk.rs
+++ b/lofty/src/iff/chunk.rs
@@ -101,6 +101,10 @@ impl<R: Read + Seek, B: ByteOrder> Chunks<R, B> {
 			"Parsed chunk header: (FourCC={}, size={size})",
 			fourcc.escape_ascii()
 		);
+		// `self.remaining_size` still includes the 8 header bytes (FourCC + size) just read above,
+		// so it has to be accounted for before comparing against the declared content size below.
+		self.remaining_size -= u64::from(IFF_CHUNK_HEADER_SIZE);
+
 		if u64::from(size) > self.remaining_size {
 			log::warn!(
 				"Chunk exceeds reader size, stopping (size: {size}, remaining: {})",
@@ -116,7 +120,6 @@ impl<R: Read + Seek, B: ByteOrder> Chunks<R, B> {
 			return Ok(None);
 		}
 
-		self.remaining_size -= u64::from(IFF_CHUNK_HEADER_SIZE);
 		self.current_chunk_size = size;
 		self.current_chunk_remaining_size = size;
```

Verified against `repro/`: before the patch, both the `size=8` and `size=5` variants of the
trailing malformed chunk produce `Err(FileParseError)`; after the patch, both produce
`Ok(WavFile { .. })` with the already-parsed `fmt`/`data` properties intact, and the malformed
trailing chunk is now cleanly stopped-at (via the existing `Ok(None)` graceful-stop path) instead
of hitting a hard I/O error.

- Project test suite (`cargo test -p lofty --lib --tests`): **646 passed, 0 failed, 73 ignored
  before the patch; 646 passed, 0 failed, 73 ignored after the patch** (identical pass/fail counts
  across all 7 test binaries: `lofty` unit tests, `tests/files`, `tests/fuzz`, `tests/hound`,
  `tests/picture`, `tests/taglib`, `tests/tags`).

## What's not done / open questions

- The same off-by-8 bug class was not independently re-verified with an AIFF-specific repro this
  session (AIFF's `read_from` uses the identical shared `Chunks::next()`, so the root cause is the
  same, but no dedicated AIFF byte-level repro was built and run).
- Only WAV's three content-consuming chunk kinds were used to trigger the observable read-time
  EOF (`fmt `/`ID3 `); the `LIST`/`fact` arms and AIFF's own chunk dispatch weren't separately
  exercised, though they all route through the same `Chunks::next()`/`Chunk::content()` machinery.
- No longer fuzzing campaign was run against this code path this session; a short `fuzz-bug-repro`
  session against `wavfile_read_from`/`aifffile_read_from` targeting chunk-size manipulation could
  turn up related boundary issues (e.g. combined with the padding logic in `correct_position`).
