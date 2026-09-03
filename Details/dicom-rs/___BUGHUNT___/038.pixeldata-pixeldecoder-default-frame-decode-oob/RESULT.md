# pixeldata-pixeldecoder-default-frame-decode-oob

`pixeldata/src/lib.rs`'s public `PixelDecoder` trait has a *default* implementation of
`decode_pixel_data_frame(&self, frame: u32)` (only `decode_pixel_data` is a required method - this
one is optional to override) that computes `frame_offset = frame_size * frame as usize` and then
directly indexes `data[frame_offset..frame_offset + frame_size]` with **no bounds check on
`frame`** before doing so - unlike the crate's own inherent `PixelData::frame_data` method and the
crate's *own* concrete override of this same method for `FileDicomObject<InMemDicomObject<D>>`
(the only type in this codebase that implements `PixelDecoder`), both of which use a graceful
`.get(..).context(FrameOutOfRangeSnafu)` pattern instead of raw indexing. Found by reading the
`PixelDecoder` trait in full after confirming candidate `037` (a different bug, in
`overlay.rs`) - this file's other size/frame arithmetic was worth a second look given `037`'s
finding, per `SKILL.md`'s "the same underlying bug can live in more than one place."

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual reading: diffed the trait's default `decode_pixel_data_frame` (line 2113) against its only real override (`FileDicomObject<InMemDicomObject<D>>`'s own implementation, line 2407) and against the inherent `PixelData::frame_data` method (line 570) - the "diff N structurally-parallel implementations of the same operation" technique already noted as high-yield in `projects/dicom-rs.md` from the prior session's `023`/`024` findings | Static code reading, no build needed | The default trait method is the only one of the three that indexes `data[a..b]` directly; the other two use `.get(a..b)` with a graceful `FrameOutOfRangeSnafu` fallback. Confirmed via `grep` that this crate has exactly one `impl PixelDecoder for` site, and it *does* override `decode_pixel_data_frame` - so the buggy default is currently dead code within this codebase itself, only reachable by an external/downstream crate that implements `PixelDecoder` for its own type without overriding this optional method (or a future maintainer of this crate who adds a new implementor and forgets to override it) |
| Hand-built boundary input: a small wrapper type (`struct DefaultFrameDecoder(DefaultDicomObject)`) implementing `PixelDecoder` with only `decode_pixel_data` delegated, deliberately not overriding `decode_pixel_data_frame` - exercising the trait's default method directly, on an ordinary single-frame real-world sample file (`dicom-test-files`' `pydicom/SC_rgb_16bit.dcm`), requesting the (out-of-range) frame index `1` | Debug build, `cargo test` (temporary unit test added to `pixeldata/src/lib.rs`, reverted before this record was written) | Panics: `range end index 120000 out of range for slice of length 60000` at `pixeldata/src/lib.rs:2125`. No extreme/adversarial attribute values needed at all - an entirely ordinary file and an entirely ordinary one-past-the-end frame request (the kind of off-by-one a caller could trivially make, e.g. iterating `0..=number_of_frames` instead of `0..number_of_frames`) triggers it. Reproduced 3/3 in-tree unit-test runs, then 2/2 more via the standalone `repro/` built against a fresh clone of pristine upstream `df2b9e42` (5/5 total) |
| Second boundary input in the same class (valid frame index `0` on the same file) | Same technique | Confirmed `decode_pixel_data_frame(0)` succeeds normally both before and after the fix - the bug is specific to out-of-range indices, not a general breakage of the method |

## Status

**Confirmed** - deterministic, pure out-of-bounds-slice panic with no layout/seed/timing
dependency; reproduces 5/5 runs (3 in-tree unit-test runs + 2 standalone `repro/` runs against a
fresh clone of pristine upstream `df2b9e42`), identical panic message and location every time.
Debug and release builds both show it identically (`data[a..b]` panics on out-of-range indices in
both profiles - this is not an overflow-checks-dependent manifestation, just a plain unchecked
slice index).

## Class

`crash` (an out-of-bounds slice index panic reachable via ordinary, ordinary-input use of a
documented public trait method - no `unsafe`, no memory-safety implication beyond the panic
itself).

## Root cause

`PixelDecoder::decode_pixel_data_frame`'s default implementation and its only real override in
this codebase (added for `FileDicomObject<InMemDicomObject<D>>`) were evidently written at
different times or by different authors and drifted out of sync on how they handle an
out-of-range `frame` argument: the override (and the crate's separate, older
`PixelData::frame_data` method) both graduated to a `.get(..).context(FrameOutOfRangeSnafu)`
graceful-error pattern, but the trait's own default implementation - which exists specifically so
that *other* implementors of `PixelDecoder` get a working `decode_pixel_data_frame` "for free"
without writing their own - was never updated to match. Since this crate's only implementor
overrides the method, the unguarded default has had no opportunity to be exercised by this crate's
own test suite or fuzz targets, letting the gap go unnoticed.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed the default implementation to use `checked_mul`/`checked_add` (saturating to `usize::MAX`
on overflow, matching candidate `037`'s fix pattern for the same reasoning: `bits_allocated`/
`samples_per_pixel`/`rows`/`cols` are attacker-controlled and `frame` is an unchecked caller
argument, so their product can also overflow `usize` for sufficiently large inputs, not just be
"merely" out of range) and `.get(..).context(FrameOutOfRangeSnafu { frame_number: frame })?`
instead of direct indexing, for both the `Cow::Owned` and `Cow::Borrowed` branches - bringing the
default implementation in line with the crate's own override and inherent method.

Verified against both boundary inputs (CONFIRMATION.md section 7): frame `0` (valid) still
succeeds identically before and after the fix; frame `1` (out of range, on a 1-frame file) now
returns `Err(FrameOutOfRange { frame_number: 1, .. })` instead of panicking.

- Project test suite (`cargo test -p dicom-pixeldata --lib`): **41 passed, 1 failed (this
  candidate's own temporary repro test, reverted before writing this record) -> 42 passed, 0
  failed** after the fix, with the temporary test included. No other test in the crate's existing
  suite was affected - expected, since the fix only changes behavior for a code path this crate's
  own types never exercise (they all use the override).

## What's not done / open questions

- This bug is currently unreachable through any type shipped by this crate itself - it only
  affects downstream/external implementors of `PixelDecoder` who rely on the default method (or a
  future internal refactor that adds a second implementor without overriding it). Still fixed here
  since it's real, shipped, unsafe-by-omission public-API code, and the fix is fully backward
  compatible (same signature, same success-path behavior, only the failure path changed from panic
  to `Result::Err`).
- Whether any other trait in this crate (or the wider workspace) has a similar
  "default-vs-override implementations of the same operation drifted apart" gap was not
  systematically re-swept this session beyond this one trait - candidates `023`/`024` from the
  prior session found the same *shape* of bug via a systematic sweep of `FileMetaAttribute`'s
  several parallel match statements; a similarly systematic sweep of every `trait` with a default
  method plus at least one override in this workspace could be worthwhile for a future session but
  was not attempted here.
- `DecodedPixelData` and `PixelDecoder` were the only types/traits examined for this specific
  "frame index bounds" concern - the encapsulated (`Codec::EncapsulatedPixelData`) and
  "Encapsulated Uncompressed" (`fragments[frame as usize]`, direct unchecked index, in the
  *override's own* code at `pixeldata/src/lib.rs` around line 2510) paths were read and found to
  delegate to already-bounds-checked per-codec `decode_frame` implementations for every
  transfer syntax with a registered decoder (confirmed for RLE via a real 2-frame test file,
  `pydicom/SC_rgb_rle_2frame.dcm`, requesting frame `2`, which correctly returned a graceful
  `FrameRangeOutOfBounds` error) - but the raw `fragments[frame as usize]` branch (reachable only
  for `Codec::EncapsulatedPixelData(None, _)`, i.e. "Encapsulated Uncompressed" specifically, a
  transfer syntax with no registered decoder function) was **not** independently exercised with an
  out-of-range frame index this session; no local sample file for that specific transfer syntax
  was available, and constructing one synthetically via the low-level fragment-sequence API was not
  completed within this candidate's time budget. This is a plausible additional bug in the same
  file worth a follow-up session - noted in `projects/dicom-rs.md`.
