# X3F decoder: unchecked `caminfo.doffset + 6` slice in raw_image() panics on a well-formed-but-bogus directory

`rawler::decoders::x3f::X3fDecoder::raw_image` (`x3f.rs:113` and `x3f.rs:125`) slices the whole
file buffer at `caminfo.doffset + 6` and at `imginfo.doffset` with no bounds check, exactly
`HEURISTICS.md`'s "index/slice arithmetic near a boundary" pattern - but one layer deeper than
candidate 003's `X3fFile::new`/`X3fDirectory::new`/`X3fImage::new` finding: it is only reachable
*after* `X3fFile::new` has already succeeded parsing a well-formed-enough directory (a valid
directory offset, valid version, a consistent entry table, and an "IMA2" entry whose own 28-byte
`X3fImage::new` fields all read successfully), but where that entry's data block happens to end
very close to the true end of the file. Candidate 003's `RESULT.md` explicitly flagged this as an
unconfirmed follow-up lead ("not independently confirmed reachable"; this candidate confirms it.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `X3fFile::new`/`X3fDirectory::new`/`X3fImage::new`/`X3fDecoder::raw_image` (`x3f.rs:39-135`) to work out exactly which byte offsets are validated before `raw_image` runs | - | `X3fImage::new`'s own field reads (`LEu32(data, 8..28)`) already force `offset + 28 <= buf.len()` to succeed (otherwise `X3fFile::new` itself panics first, one layer up - the class candidate 003 already fixed). But `doffset = offset + 28` is stored as-is with no further margin, so `X3fFile::new` succeeding only guarantees `caminfo.doffset <= buf.len()`, not `caminfo.doffset + 6 <= buf.len()` - the `+6` at `x3f.rs:113` is the actual gap |
| Manual boundary-value construction: a byte-exact X3F file (magic, directory header, one directory entry, one 28-byte "IMA2" image block whose fields declare `typ=2, format=0x12`) sized so the image block's last byte lands exactly at the file's true end, i.e. `caminfo.doffset == file length` (so `X3fFile::new`/`X3fImage::new` succeed cleanly) but `caminfo.doffset + 6` is 2 bytes past the end | 64-byte crafted file, debug and release builds | Deterministic panic at `x3f.rs:113`, both builds |
| Standalone repro: the crafted 64-byte file written to a real file on disk, opened via `rawler::imgop::develop::process_raw_image` - the real, public, top-level function `bin/dnglab/dnglab-lib/src/jobs/process_raw.rs`'s `Raw2Image` job calls to convert a raw file (this function itself calls `rawler::get_decoder` then `decoder.raw_image()` directly, with no panic-catching wrapper, exactly mirroring the dnglab CLI's own code path) | debug build, 3 runs; release build, 1 run; plus a second, independent run of the full `just test` cycle (fresh `git clone` of this repo at the recorded `base_commit`, release build) | Deterministic panic, 3/3 debug runs + 1/1 release run + 1/1 `just test` run, identical panic message and location (`x3f.rs:113:23`, "range start index 66 out of range for slice of length 64") each time |
| Boundary-value sweep after applying the fix: the original 64-byte crash input (`caminfo.doffset+6` 2 bytes past end), a variant with no padding after the image block (same OOB), a variant with 10 bytes of trailing padding so `caminfo.doffset+6` is in-range but no `"Exif"` magic present, and a variant with in-range offset AND a valid `"Exif"` magic byte sequence | debug build | First two: clean `Err("X3F: Camera info offset out of range")`, no panic. Third: falls through correctly to the pre-existing `"X3F: Couldn't find EXIF info"` check (proves the fix doesn't reject genuinely in-range offsets). Fourth: proceeds past the fix into the pre-existing IFD/camera-lookup logic, failing later only because no real TIFF/IFD tags were crafted (`"Couldn't find tag TiffCommonTag::Make"`) - confirms the fix is not overly strict |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after |

## Status

**Confirmed.** Reproduces 3/3 debug runs, 1/1 release run, and 1/1 independent `just test` run from
a fresh clone - fully deterministic (the panic message includes the literal byte offsets computed
from the fixed 64-byte crafted input, not layout/seed-dependent).

**Fully reproducible**: **Yes**. `repro/` builds the crafted X3F file, writes it to a real file on
disk, and opens it through the unmodified, public `rawler::imgop::develop::process_raw_image` -
the exact function dnglab's own `Raw2Image` job calls to process a raw file, with no
panic-catching wrapper in between (unlike `rawler::decoders::RawLoader::decode`, which does wrap
decoding in `panic::catch_unwind`; `process_raw_image` and the `get_decoder` + `raw_image()`
pattern used directly by `bin/dnglab/dnglab-lib/src/jobs/process_raw.rs` and
`bin/dnglab/dnglab-lib/src/makedng.rs` do not). Single-participant case (a malformed file handed
to a parser's real public entry point) - no second participant to fake, per `CONFIRMATION.md`
section 11.

## Class

`crash`.

## Root cause

`X3fDecoder::raw_image` (`x3f.rs:113`):

```rust
let data = &buffer[caminfo.doffset + 6..];
```

`caminfo.doffset` is set by `X3fImage::new` as `offset + 28` (`x3f.rs:85`), where `offset` is a
directory entry's own attacker-controlled `off` field. `X3fImage::new` itself only proves
`offset + 28 <= buf.len()` (its own field reads go up to byte 27 of the 28-byte block) - it proves
nothing about `offset + 28 + 6`. Since `raw_image` re-adds `+ 6` on top of the already
maximally-tight `doffset` with no further bounds check, any directory entry whose 28-byte image
block ends within the last 6 bytes of the file (a range of exactly 6 possible byte lengths, `0`
through `5` bytes short of the `+6`) panics with an out-of-range slice index - reachable only
*after* `X3fFile::new` has already succeeded, so it survives every check candidate 003's fix (not
applied to this candidate, applied only in that candidate's own patch) adds to the earlier parsing
stage. `imginfo.doffset` at `x3f.rs:125` (`let src = &buffer[offset..];`) has the same *shape* of
unchecked slice, but since `offset` there is used with no further addition, `X3fFile::new`
succeeding already guarantees `offset <= buf.len()` - so this second location was not
independently observed to panic on its own; it was hardened defensively (see "The fix") as the
same bug class, not because a standalone repro was built for it.

## Introduced by

7f34dcdf "Add 'rawler/' from commit 'c793a132fa03e336f8c5c078f01d3ed1a3ee8c07'" (the commit that
subtree-imported the `rawler/` crate's entire pre-existing history into this repository).
`git log -S"doffset+6" --all -- rawler/src/decoders/x3f.rs` and `git show
7f34dcdf:rawler/src/decoders/x3f.rs` both confirm the exact `self.buffer[caminfo.doffset+6..]`
unchecked-slice shape is already present in the very first revision of this file that exists
anywhere in this repository's tracked history - it predates every later commit that touched this
file (including 416d897a's later reformatting pass, and candidate 003's own investigated commit
history), and was not introduced by any change made within this repository. The bug is original to
the external `rawler` project this repo imported, not a regression introduced here.

## The fix

Replaced the unchecked `&buffer[caminfo.doffset + 6..]` slice with a `checked_add(6)` followed by
`buffer.get(start..)`, additionally requiring the resulting slice have at least 4 bytes (the
minimum `data[0..4]` read that follows immediately needs), turning any out-of-range or
too-short-to-check offset into the function's existing `Result` error path (consistent with how
this file already reports other malformed-input conditions, e.g. "X3F: Couldn't find EXIF info").
Also hardened the second, structurally identical unchecked slice on `imginfo.doffset`
(`&buffer[offset..]` -> `buffer.get(offset..).ok_or(...)?`) defensively, even though it was not
independently confirmed to panic on its own this session (see "Root cause" above) - it shares the
exact "attacker-influenced offset used to slice the full file buffer with no bounds check" class
this whole file already has two other instances of (candidate 003) and one more here.

Verified: `repro/`'s crafted 64-byte input no longer panics after the fix and returns a
descriptive error instead (`"X3F: Camera info offset out of range"`), in both debug and release,
and via a fresh `just test` clone-apply-reproduce cycle. Re-verified against 2 additional boundary
variants in the same bug class (an in-range offset with no "Exif" magic, and an in-range offset
with a valid "Exif" magic prefix) - both confirm the fix rejects only genuinely out-of-range
offsets and lets in-range input continue exactly as it did before the fix.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no existing test exercises the X3F decoder's `raw_image` path
  with malformed input, so this fix is additive coverage, not a regression risk.

## What's not done / open questions

- The `imginfo.doffset` hardening (`x3f.rs:125`, the second half of this candidate's original
  description) was fixed defensively but not independently confirmed to be reachable as its own
  standalone panic this session - see "Root cause" above for why (`X3fFile::new` succeeding
  already guarantees that particular offset is in-bounds, unlike `caminfo.doffset + 6`). If a
  future session wants to confirm it independently, the only way it could panic is via a
  `decode_compressed` path reading past `src`'s end for a given `width`/`height` (a different bug
  class - unvalidated width/height vs. available data - not the "raw offset used to slice the
  buffer" class this candidate is about), since `X3fImage::format` in this repo's tree only
  supports `format == 35` (LJPEG-compressed) and `decode_compressed` is not fully implemented
  (`x3f.rs:156` returns `Err("X3F decoding not implemented yet")` unconditionally in this version)
  - so the `src` slice itself is never actually read by any currently-implemented decode path.
- The same "well-formed directory, bogus-but-internally-consistent per-entry data" input class
  used here (a directory entry whose declared fields are all self-consistent but whose *implied*
  extent runs past the buffer) was not swept across other decoders in this session - this
  candidate is scoped to X3F only.
