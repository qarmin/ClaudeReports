# FLAC_SEEKTABLE_BYTE_OFFSET_ADD_OVERFLOW

`cargo clippy --pedantic` flags `(a + b) / 2` midpoint computations in binary-search seek code as
`clippy::manual_midpoint` in `symphonia-bundle-flac/src/demuxer.rs` and
`symphonia-format-ogg/src/{demuxer,physical}.rs` (previously recorded as a low-priority lead in
`projects/symphonia.md`, judged "not exploitable in practice" since real byte offsets are bounded
by actual file size, far short of the ~9.2 exabytes/side needed for `u64::MAX` addition overflow).
This candidate set out to verify that judgment by checking actual bounds/types at each site, and
whether any nearby arithmetic on the *same* values is fed by something less trustworthy than "an
actual, filesystem-bounded byte offset".

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of all 3 flagged sites plus `symphonia-core/src/formats/util.rs`'s own `SeekIndex::search` midpoint and `symphonia-format-isomp4/src/atoms/stsc.rs`'s (already overflow-safe) `left + (right - left) / 2` | Traced every operand's actual type and provenance | `symphonia-format-ogg`'s two sites and `symphonia-core`'s `SeekIndex::search` midpoint operate on values genuinely bounded by real stream/vec size (confirmed non-issue, matches the prior judgment). `symphonia-bundle-flac/src/demuxer.rs`'s `do_seek` is different: `start_byte_offset`/`end_byte_offset` are computed as `self.first_frame_offset + <SeekPoint>::byte_offset`, and `byte_offset` is a raw `u64` read directly off the FLAC SEEKTABLE metadata block by `symphonia-metadata/src/embedded/flac.rs::read_flac_seektable_block` with **no validation whatsoever** against the actual stream length - fully attacker-controlled, unlike a "real" byte offset |
| Debug-assertions build (`cargo build`, default `dev` profile, `overflow-checks=true` by cargo's own default) + manual boundary-value construction: hand-crafted 2 real, format-valid FLAC files (marker + STREAMINFO + SEEKTABLE with 1 or 2 seek points, `byte_offset` set to `0xFFFFFFFFFFFFFFFF` or `~u64::MAX/2` respectively), read through the public `FlacReader::try_new` + `FormatReader::seek` API | Debug build, 2 distinct crafted `.flac` files (see `repro/assets/`), `seek(SeekMode::Coarse, SeekTo::Timestamp{ts, ..})` | Reproduced: **`seek_overflow_real.flac`** (1 seek point, `byte_offset=u64::MAX`) panics `attempt to add with overflow` at demuxer.rs:305 (`start_byte_offset = self.first_frame_offset + lower.byte_offset;`, the `SeekSearchResult::Lower` arm) before the midpoint line is ever reached. **`seek_overflow_midpoint.flac`** (2 seek points, `byte_offset ≈ 0xC000000000000000` each) panics at the literal flagged line, demuxer.rs:322 (`let mid_byte_offset = (start_byte_offset + end_byte_offset) / 2;`) - both individual `first_frame_offset + byte_offset` additions stay in range, but their *sum* in the midpoint expression itself overflows, which is the exact `clippy::manual_midpoint` pattern this candidate was asked to check, now shown reachable via crafted seek-table bytes rather than an implausible multi-exabyte real file |
| Repeat-run + release-mode check | Same debug binary, same pristine tree, 5 consecutive runs of `seek_overflow_midpoint.flac` | 5/5 runs panic identically at demuxer.rs:322. A `--release` build (cargo default, `overflow-checks=false`) does **not** panic for either file: the addition/midpoint silently wrap, and the resulting huge wrapped byte offset is passed to `self.reader.seek(SeekFrom::Start(mid_byte_offset))`, which the OS/`File` seek implementation then rejects with an ordinary `Err` (`Invalid argument (os error 22)` or `unexpected end of file`) - no crash, no memory-safety issue in release mode, just checked-arithmetic panics specific to debug/overflow-checks-enabled builds |

## Status

**Confirmed**. Reproduces 5/5 runs, deterministic, debug build (cargo's own default
`overflow-checks=true` for the `dev` profile - no special `RUSTFLAGS` needed). A debug-assertions
build plus manual boundary-value construction was sufficient per `HEURISTICS.md`'s guidance for
this arithmetic-on-untrusted-values bug class; no sanitizer/miri/fuzzing needed.

**Fully reproducible**: **Yes**. Both repro files are real, format-valid FLAC files (genuine
`fLaC` marker + STREAMINFO + SEEKTABLE metadata blocks, `seek_overflow_midpoint.flac` derived by
splicing a crafted SEEKTABLE block into an actual ffmpeg-encoded FLAC file), driven entirely
through the public `FlacReader::try_new` + `FormatReader::seek` API - exactly how a real player
would open and seek a file. Per `CONFIRMATION.md` section 11, a malformed file handed to a
parser's real public entry point is the genuine external input; there is no second participant to
fake here.

## Class

crash (checked-arithmetic overflow panic, `overflow-checks=true` builds only)

## Root cause

`FlacReader::do_seek`'s seekable binary-search bisection (`symphonia-bundle-flac/src/demuxer.rs`,
~lines 292-323) computes `start_byte_offset`/`end_byte_offset` from
`self.first_frame_offset + <SeekPoint>::byte_offset` whenever `self.index` (built from the FLAC
file's own optional SEEKTABLE metadata block) has a usable entry for the target timestamp.
`SeekPoint::byte_offset` is populated by `read_flac_seektable_block`
(`symphonia-metadata/src/embedded/flac.rs:137-168`), which reads it as a raw big-endian `u64`
straight off the file with **no bounds check** against the actual stream length or against
`first_frame_offset` - unlike the `end_byte_offset` variable's *other* source
(`self.reader.seek(SeekFrom::End(0))?`, the real, filesystem-bounded stream length), a SEEKTABLE
entry's `byte_offset` is exactly as attacker-controlled as any other file field. This breaks the
original clippy-lint-driven judgment's premise ("real byte offsets bounded by actual file size") -
the values feeding `(start_byte_offset + end_byte_offset) / 2` (and the addition/subtraction
around it) are not bounded by anything at all until this point in the code. Two distinct panic
sites result from the same missing check: the `self.first_frame_offset + lower.byte_offset`/
`upper.byte_offset` additions themselves (lines 301/305/309/310), and, once those individually
survive, the loop's own midpoint/subtraction (`end_byte_offset - start_byte_offset` at line 321,
`(start_byte_offset + end_byte_offset) / 2` at line 322).

## Introduced by

c9612e0 "Implement seeking for FLAC." (2019-03-05) - the original commit that introduced FLAC
seeking (in what was then `sonata-codecs-flac/src/lib.rs`, since split into
`symphonia-bundle-flac/src/demuxer.rs` by 565d186 in the same era, then renamed/relocated through
the project's `sonata` -> `Symphonia` rename). `git blame` on the current
`start_byte_offset = self.first_frame_offset + lower.byte_offset` line and its siblings attributes
them to 565d186 "Split FLAC decoder and demuxer into seperate modules." (2019-04-13), a pure
code-motion commit; the actual arithmetic (and the missing bounds check) traces back to c9612e0,
which the `--follow`-aware log confirms is where FLAC seeking - and this exact unchecked-addition
pattern - was first written. The SEEKTABLE-parsing side (`read_flac_seektable_block`) was moved
into its current file/location by 2c85def6 (2024-11-05) and lightly reformatted by 666b2132
(2026-02-07); neither of those commits changed the missing-validation behavior, both parsing the
`byte_offset` field the same trusting way it always has been.

## The fix

`patch.diff` changes `FlacReader::do_seek` to:

1. Compute `start_byte_offset`/`end_byte_offset` from a SEEKTABLE entry's `byte_offset` with
   `self.first_frame_offset.saturating_add(byte_offset)` instead of a raw `+`, avoiding the
   addition-overflow panic outright.
2. Clamp the saturated result to `.min(stream_end_offset)` (the real, already-known-correct
   stream length obtained by `self.reader.seek(SeekFrom::End(0))?` at the top of the function),
   so an untrusted seek-table entry can never push `start_byte_offset`/`end_byte_offset` outside
   the file's real byte range - and, since both are now bounded by the same `stream_end_offset`
   and `start_byte_offset`'s floor (`first_frame_offset`) is always `<= stream_end_offset`, this
   also guarantees `start_byte_offset <= end_byte_offset`, ruling out the sibling subtraction-
   underflow at the `while end_byte_offset - start_byte_offset > ...` loop condition.
3. As defense in depth (and to actually resolve the `clippy::manual_midpoint` lint this candidate
   started from), rewrites the midpoint expression itself from `(start + end) / 2` to the
   overflow-safe `start + (end - start) / 2` - with (1)/(2) in place this is no longer reachable
   with realistic inputs, but it removes the lint and matches the pattern
   `symphonia-format-isomp4/src/atoms/stsc.rs` already uses.

Verified by re-running `repro/` against the patched tree: both `seek_overflow_real.flac` and
`seek_overflow_midpoint.flac` now return an ordinary `Err("unexpected end of file")` instead of
panicking (5/5 runs each, debug build). See `output.txt` for the real `just test` output showing
both panics before the patch and both clean `Err` results after.

- Project test suite (`cargo test -p symphonia-bundle-flac -p symphonia-metadata -p
  symphonia-core`): **79 passed, 0 failed** before the patch, **79 passed, 0 failed** after
  (2 in symphonia-bundle-flac, 65 in symphonia-metadata, 12 in symphonia-core; identical counts,
  no new failures).

## What's not done / open questions

- The other 2 originally-flagged `clippy::manual_midpoint` sites (`symphonia-format-ogg/src/
  demuxer.rs:192`, `symphonia-format-ogg/src/physical.rs:130`) and `symphonia-core/src/formats/
  util.rs:120`'s own index-search midpoint were checked by code reading only (not built/run): all
  three operate on values genuinely bounded by real stream size or real `Vec` length (Ogg's
  `byte_range_start`/`byte_range_end` come from `self.reader.seek(SeekFrom::End(0))`-style calls
  or from recursive bisection of those, never from an unchecked file-supplied field the way FLAC's
  SEEKTABLE `byte_offset` does; `SeekIndex::search`'s `lower`/`upper` are `usize` indices into an
  in-memory `Vec`, bounded by actual allocated memory) - the original "not exploitable, needs
  multi-exabyte files" judgment stands for those three specifically, this candidate's finding is
  narrowly about FLAC's SEEKTABLE-derived offsets, not the midpoint pattern in general.
- Whether other SEEKTABLE-adjacent or seek-index-adjacent consumers of `SeekPoint::byte_offset`
  elsewhere in the codebase have the same missing-validation pattern was not exhaustively swept -
  `symphonia-core::formats::util::SeekIndex` is a shared, generic type; any other format reader
  that builds one from an unvalidated file-supplied byte offset (not checked this session for
  formats other than FLAC) could have the same class of bug.
