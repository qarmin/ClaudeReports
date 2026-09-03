# jpeg_precision_components_overflow

`lofty/src/picture/mod.rs:474`, `PictureInformation::from_jpeg` (lines 430-483): a JPEG `SOF0`/
`SOF2` marker's `precision` and `components` fields are both read as `u8`, then multiplied
*before* being widened to `u32` (`u32::from(precision * components)`), so `precision * components`
overflows whenever their product exceeds 255. This candidate came from a prior static-review pass
recorded in `IDEAS.md` (a temporary in-tree `#[test]`, since reverted, had already shown the
overflow panics with a hand-built byte buffer fed directly to `from_jpeg`). This session built and
ran a real, end-to-end reproduction through lofty's public API to confirm it.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Standalone repro via the real public API end-to-end: write a real ID3v2 `APIC` frame (containing a malformed JPEG `SOF0` marker, `precision = 0xFF`, `components = 0xFF`) into a real MP3 file with `Tag::save_to_path`, read it back from disk with `lofty::read_from_path`, then trigger the real, public `From<Tag> for VorbisComments` conversion (the code path lofty itself uses to carry a `Tag`'s pictures into a FLAC/OGG file) | default cargo `dev` profile (`overflow-checks = true`, the project's own default build/test config) | Reproduced: `thread 'main' panicked at .../picture/mod.rs:474:44: attempt to multiply with overflow` |
| Repeat-run check | `dev` profile, same input, 3 runs | 3/3 - fully deterministic, no build-flag/layout/seed dependence |
| Release build of the same repro | `cargo run --release` (`overflow-checks = false`) | No panic - the multiplication silently wraps (`0xFF * 0xFF mod 256 = 1`), producing a wrong `color_depth` of `1` instead of the correct `65025` with no error surfaced |
| Second boundary value in the same class (`precision = 0x10`, `components = 0x20`, product `512`, a different overflow magnitude than `0xFF * 0xFF = 65025`) | `dev` profile, same end-to-end harness | Reproduced identically before the fix (panic), and confirmed fixed after the fix (no panic) |

## Status

**Confirmed**. Deterministic 3/3 `dev`-profile repeat runs on the exact triggering input, plus a
second boundary value in the same overflow class, reproduced identically before the fix and
resolved after it. The project's own default `cargo test`/`cargo run` profile has
`overflow-checks = true` (no override in the workspace `Cargo.toml`), so this panics under the
project's own normal dev/test builds, not only under a special sanitizer configuration; a release
build (`overflow-checks = false`) does not panic but silently produces a wrong `color_depth` value.

**Fully reproducible**: **Yes**. The repro writes a real ID3v2 `APIC` frame (containing the
malformed JPEG) into a real MP3 file using lofty's own public writer (`Tag::save_to_path`), then
reads that file back from disk through lofty's real public reader (`lofty::read_from_path`) -
exercising the actual `AttachedPictureFrame::parse` path a caller opening an untrusted MP3 would
go through - and finally exercises the real, public `From<Tag> for VorbisComments` conversion,
which is what actually calls `PictureInformation::from_picture` -> `from_jpeg` on the untrusted
bytes that just came off disk. This is the single-participant case `CONFIRMATION.md` section 11
explicitly calls out as needing no second-participant repro: a malformed file (the MP3 with its
crafted `APIC` picture) handed to a parser's real public entry point *is* the genuine external
input, the same bytes a real attacker/user-supplied file would contain.

## Class

crash (debug/overflow-checked builds) / logic (release builds, silent wrong `color_depth`)

## Root cause

`PictureInformation::from_jpeg` reads a JPEG frame header's `precision` (1 byte) and `components`
(1 byte) fields as `u8`, then computes `precision * components` as a `u8` multiplication before
widening the *result* to `u32` via `u32::from(...)`. Since both operands can independently reach
`0xFF`, their product can be as large as `65025`, far exceeding `u8::MAX` (255). Under the
project's own default `overflow-checks = true` dev/test profile this panics with "attempt to
multiply with overflow"; under a release build it silently wraps modulo 256, producing an
incorrect `color_depth` with no indication anything went wrong.

`from_jpeg` is reached via `PictureInformation::from_picture`, which is called by:
- `OggPictureStorage::insert_picture` (`lofty/src/ogg/picture_storage.rs:38`) whenever a picture
  is inserted without an explicit `PictureInformation`.
- `MergeTag for SplitTagRemainder` (`lofty/src/ogg/tag.rs:670`, used by `From<Tag> for
  VorbisComments`) and `create_vorbis_comments_ref` (`lofty/src/ogg/tag.rs:753`), which recompute
  `PictureInformation` for every picture in a generic `Tag` whenever that `Tag` is converted to or
  written as `VorbisComments` (i.e. whenever a `Tag`'s pictures - which may have originated from
  an ID3v2 `APIC` frame, an APE binary item, or an MP4 `covr` atom, none of which carry a
  precomputed `PictureInformation` - are carried into a FLAC/OGG file).

FLAC's and ID3v2's own picture formats (`METADATA_BLOCK_PICTURE`, `APIC`) do *not* call
`from_jpeg` on read - they carry `width`/`height`/`color_depth`/`num_colors` as explicit fields in
the container. The overflow is reached specifically on the *cross-format* path: an untrusted
picture that arrived via a format without an embedded `PictureInformation` (ID3v2 `APIC`, APE
binary items, MP4 `covr`) being converted into `VorbisComments` for a FLAC/OGG file, where lofty
computes `PictureInformation` itself by actually parsing the image bytes.

## Introduced by

bae894cc "Add PictureInformation parsing for PNG and JPEG, remove PictureInformation from Picture" (2021-11-22) - `git log -S"precision * components"` finds this as the commit that introduced `from_jpeg`'s `u8` multiplication, unchanged in shape since.

## The fix

Widen `precision` and `components` to `u32` *before* multiplying, instead of after:

```rust
// before
color_depth: u32::from(precision * components),
// after
color_depth: u32::from(precision) * u32::from(components),
```

Since `u8::MAX * u8::MAX = 65025` fits comfortably in `u32`, this can never overflow for any
`u8` input, so the fix is general rather than tailored to the one triggering value pair - verified
against both the original `0xFF`/`0xFF` input and a second boundary pair (`0x10`/`0x20`, a
different overflow magnitude) in the same class; both reproduced the panic before the fix and were
resolved after it.

- Project test suite (`cargo test -p lofty`): identical results before and after the fix - `256
  passed; 0 failed` (lib unit tests), `122 passed; 0 failed` (`tests/files`), `56 passed; 0 failed`
  (`tests/fuzz`), `2 passed; 0 failed` (`tests/hound.rs`), `15 passed; 0 failed`
  (`tests/picture`), `191 passed; 0 failed; 73 ignored` (`tests/taglib`), `4 passed; 0 failed`
  (`tests/tags`), plus `176 passed; 0 failed; 23 ignored` doctests. No new failures introduced.

## What's not done / open questions

- `PictureInformation::from_png` (`lofty/src/picture/mod.rs:348`) was not specifically re-checked
  for the same overflow class - its color-depth arithmetic (`color_depth *= 3` / `color_depth *=
  4`) widens to `u32` before multiplying, so it does not look susceptible to this exact pattern,
  but it was not independently fuzzed or boundary-tested this session.
- The existing fuzz target `fuzz/fuzz_targets/pictureinformation_from_jpeg.rs` calls `from_jpeg`
  directly (a single-participant harness, consistent with `CONFIRMATION.md` section 11's
  exception) and would likely have found this quickly with `overflow-checks` enabled in the fuzz
  profile; not run this session as a separate step since the end-to-end repro above already gives
  a `Confirmed` result via the real cross-format conversion path.
- Other numeric fields read directly from untrusted picture bytes elsewhere in `picture/mod.rs`
  (e.g. `from_png`'s chunk-size arithmetic) were read but not independently boundary-tested this
  session beyond the one candidate this session focused on.
