# aiff_chunk_off_by_8

`lofty/src/iff/aiff/read.rs`'s `AiffFile::read_from` drives the same shared `Chunks::next()` in
`lofty/src/iff/chunk.rs` whose off-by-8 bounds-check bug was already confirmed and fixed for WAV
in `___BUGHUNT___/001.iff_chunk_off_by_8/` (a prior session's `RESULT.md` explicitly left this
follow-up open: "not independently re-verified with an AIFF-specific repro"). This session builds
and runs a real, standalone AIFF-specific repro to get an independent, format-specific
confirmation of the same defect.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + standalone repro via public API (`AiffFile::read_from`) | release build, in-memory `Cursor<Vec<u8>>` AIFF: valid `COMM` + valid `SSND` chunks followed by a trailing `ID3 ` chunk header declaring 8 content bytes with 0 physically present | Reproduced: `Err(FileParseError)` on unpatched code, `Ok(AiffFile)` on patched code |
| Same repro, second boundary value in the class (declared size 5 instead of 8, both within the 1..=8 off-by window) | release build, same harness | Reproduced identically (`Err` before / would be `Ok` after) |
| Repeat-run check | `RUNS=5`, release build, same input (declared size 8) | 5/5 - fully deterministic, no build-flag/layout/seed dependence |

## Status

**Confirmed**. Deterministic 5/5 release-build repeat runs on the exact triggering input, plus a
second boundary value (declared overshoot of 5 instead of 8) in the same off-by-window class also
reproduced deterministically.

**Fully reproducible**: **Yes**. The repro calls `AiffFile::read_from` (the real, public
`AudioFile::read_from` entry point) directly on a malformed byte buffer - a malformed file handed
to a parser's public entry point *is* the genuine external input, with no second participant to
fake (per `CONFIRMATION.md` section 11's explicit "single-process, single-participant repro
remains completely fine" case).

## Class

logic

## Root cause

Identical mechanism to `001.iff_chunk_off_by_8/`, since AIFF and WAV parsing share the exact same
`Chunks::next()`/`Chunk` code in `lofty/src/iff/chunk.rs`: the bounds check
`if u64::from(size) > self.remaining_size` runs before `self.remaining_size -=
u64::from(IFF_CHUNK_HEADER_SIZE)` accounts for the 8 header bytes (FourCC + size) just consumed
off the reader. At the moment of the check, `self.remaining_size` still counts those 8 bytes as if
they were still available as chunk content, so a chunk whose declared `size` overshoots the truly
remaining content bytes by up to 8 is wrongly accepted.

For AIFF specifically: `AiffFile::read_from` builds a `Chunks::<_, BigEndian>::new(data, file_len)`
and loops `chunks.next(parse_options.parsing_mode)`. Once the malformed trailing `ID3 ` chunk is
wrongly accepted by `next()`, the `b"ID3 " | b"id3 "` match arm in `read_from` calls
`chunk.id3_chunk(parse_options)`, which calls `self.content()` -> `read_exact` on a buffer sized to
the (over-declared) chunk size. That `read_exact` hits genuine physical EOF and returns a hard
`std::io::ErrorKind::UnexpectedEof`, mapped through `TagParseError::from` and propagated via `?`
all the way out of `read_from` as `Err(FileParseError)` - even though the caller requested the
default `ParsingMode::BestAttempt`, whose entire purpose is to tolerate exactly this kind of
truncated/corrupt trailing data and keep whatever was already parsed successfully (the valid
`COMM`/`SSND` chunks in the repro, which would otherwise have produced a complete `AiffProperties`).

## Introduced by

b64f9722 "IFF: Support invalid chunk padding" (2026-02-27) - same commit and mechanism as sibling candidate 001, since AIFF and WAV share the exact same `Chunks::next()` code in `lofty/src/iff/chunk.rs`.

## The fix

Same fix as `001.iff_chunk_off_by_8/`, applied to the same shared function (this candidate did not
need to draft a new fix - it reused and re-verified the already-drafted one against an
AIFF-specific repro): move the `self.remaining_size -= u64::from(IFF_CHUNK_HEADER_SIZE)`
subtraction to run *before* the `size > self.remaining_size` comparison, instead of after it, so
the check compares the declared content size against the bytes truly available for content (the
earlier `if self.remaining_size < u64::from(IFF_CHUNK_HEADER_SIZE) { return Ok(None) }` guard a few
lines above already guarantees this subtraction can't underflow).

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

Verified against `repro/`: before the patch, the AIFF repro (declared trailing-chunk size 8, 0
bytes physically present) produces `Err(FileParseError)`; after the patch, it produces
`Ok(AiffFile { properties: AiffProperties { .. } })` with the already-parsed `COMM`/`SSND`
properties intact, and the malformed trailing chunk is now cleanly stopped-at (via the existing
`Ok(None)` graceful-stop path) instead of hitting a hard I/O error.

- Project test suite (`cargo test -p lofty --lib --tests`): **646 passed, 0 failed, 73 ignored
  before the patch; 646 passed, 0 failed, 73 ignored after the patch** (identical pass/fail counts
  across all 7 test binaries: `lofty` unit tests, `tests/files`, `tests/fuzz`, `tests/hound`,
  `tests/picture`, `tests/taglib`, `tests/tags`). One transient, unrelated failure
  (`probe::tests::probe_mp3_with_lots_of_junk`) was observed on a single run before this baseline
  was captured; a clean rerun on the same (pre-fix) tree passed, and the same test passed
  identically on both the before and after runs used for this candidate's official counts, so it
  was not caused by this change.

## What's not done / open questions

- Only AIFF's `ID3 `/`id3 ` chunk-dispatch arm was used to trigger the observable read-time EOF;
  the `COMT`/`NAME`/`AUTH`/`(c) ` text-chunk arms and the `COMM`/`SSND` arms weren't separately
  exercised as the *trailing* malformed chunk, though they all route through the same
  `Chunks::next()`/`Chunk::content()` machinery and would be expected to behave identically.
- No fuzzing campaign was run against this code path this session; a short `fuzz-bug-repro`
  session against `aifffile_read_from` targeting chunk-size manipulation could turn up related
  boundary issues, same as noted for WAV in `001`.
