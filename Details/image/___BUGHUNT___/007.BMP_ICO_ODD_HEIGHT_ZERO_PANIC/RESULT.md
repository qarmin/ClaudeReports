# BMP-in-ICO odd-height panic in read_metadata_in_ico_format

`BmpDecoder::read_metadata_in_ico_format` (`src/codecs/bmp/decoder.rs`) halves `self.height` right
after `read_metadata()` has already validated the *doubled* height, to undo the ICO
double-height convention. An odd doubled height integer-divides to 0, a value that was never
itself validated, and the pixel readers assume `height >= 1`. Selected from an externally-supplied
candidate description that named the exact file/line and mechanism (and cited an open, not yet
merged upstream PR, image-rs/image#3041, describing the same fix approach).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: hand-crafted a ~70-byte single-entry ICO whose embedded bare `BITMAPINFOHEADER` (24bpp, `BI_RGB`, width=1) declares a doubled height of 1 (odd), fed to the public `image::load_from_memory` entry point (debug build, `cargo run` in a throwaway crate against the main checkout) | `image::load_from_memory(&odd_height_ico_bytes)` | **Panicked**: `src/codecs/bmp/decoder.rs:2071:53: called Result::unwrap() on an Err value: TryFromIntError(NegOverflow)` inside `read_full_byte_pixel_data` (the RGB24 pixel reader; same `(self.height - 1).try_into::<u32>().unwrap()` pattern the candidate cited at line 1870 in a different pixel reader). Reproduced 3/3 runs, deterministic, no randomness/layout dependence (plain integer-division/validation-ordering bug, not memory-unsafety - no need to escalate to miri/ASAN/fuzzing for this bug class). |
| Same repro, control case: a *valid* doubled height of 2 (real height 1, even) through the same code path | `image::load_from_memory(&valid_ico_bytes)` | Decoded successfully as a 1x1 image, both before and after the fix - confirms the fix (below) doesn't overreject legitimate ICO-embedded BMPs. |

Only one approach round was needed (soft cap of 3 not exceeded) - this is a plain
validation-ordering/arithmetic bug (integer division producing an unvalidated 0), immediately and
deterministically reproducible by hand-constructing the boundary-value input; no sanitizer/miri/
fuzzing needed for this class.

## Status

**Confirmed.** Reproduces 3/3 runs (debug build, deterministic - no randomness or memory-layout
dependence involved) via the standalone `repro/` run both directly against the main checkout and,
separately, end-to-end via `just test` against a fresh clone at `base_commit`.

**Fully reproducible: Yes.** The repro calls the real public entry point (`image::load_from_memory`)
on a hand-crafted ICO file - a single external artifact handed to a parser's real public entry
point, the same bytes a real attacker/user would supply. There is no second participant/peer/wire
exchange involved in this finding (it's not about how one process reacts to another process's live
behavior), so `CONFIRMATION.md` section 11's "real second participant" requirement does not apply;
a single-process repro against the real decode entry point is the correct and sufficient shape.

## Class

crash

## Root cause

`BmpDecoder::read_metadata_in_ico_format`:

```rust
pub fn read_metadata_in_ico_format(&mut self) -> ImageResult<()> {
    self.no_file_header = true;
    self.add_alpha_channel = true;
    self.read_metadata()?;

    // The height field in an ICO file is doubled to account for the AND mask
    // (whether or not an AND mask is actually present).
    self.height /= 2;
    Ok(())
}
```

`read_metadata()` (via `read_bitmap_info_header`/`read_bitmap_core_header`/etc.) calls
`check_for_overflow(self.width, self.height, ...)`, which rejects `height <= 0` - but at that point
`self.height` still holds the *doubled* value straight from the file, so any positive value (odd or
even) passes. Only afterwards does `read_metadata_in_ico_format` divide it by 2 to recover the real
height, per the ICO double-height convention. Integer division of an odd doubled height (e.g. 1)
produces 0, which is never itself checked - `check_for_overflow` already ran, and nothing re-checks
`self.height` after the halving.

Every pixel reader (`read_palettized_pixel_data`, `read_full_byte_pixel_data`,
`read_16_bit_pixel_data`, `read_32_bit_pixel_data`, ...) unconditionally computes
`let last_row: u32 = (self.height - 1).try_into().unwrap();` near its start, before any row is
actually processed - so even though a 0-height image has nothing to decode, this line still runs
and panics: `self.height - 1` is `-1` as `i32`, and `TryInto::<u32>::try_into(-1)` fails, and the
result is `.unwrap()`-ed. Standalone (non-ICO) BMPs never hit this because `check_for_overflow`
sees the real (non-doubled) height directly and already rejects `height == 0`; only the
ICO-specific post-validation halving can reintroduce a zero height after validation has already
passed.

The `IcoDecoder` itself constructs the inner `BmpDecoder` eagerly (`IcoDecoder::new` ->
`DirEntry::decoder` -> `BmpDecoder::new_with_ico_format` -> `read_metadata_in_ico_format`), so
`self.height == 0` is already latched in before any dimension-mismatch check the ICO layer might
otherwise apply - and that ICO-level `matches_dimensions` check only runs in
`SpecCompliance::Strict` mode anyway, so the default lenient `image::load_from_memory` path never
even reaches it before the panicking pixel reader runs.

## Introduced by

281b5e6 "Add ICO icon file decoding" (2015-10-05)

## The fix

Added a check for `self.height == 0` immediately after the halving in
`read_metadata_in_ico_format`, returning the existing `DecoderError::InvalidHeight` (already used
elsewhere in this file for the `i32::MIN` unrepresentable-height case, so this reuses an existing,
appropriately-named error variant rather than adding a new one):

```rust
self.height /= 2;
// An odd doubled height (e.g. 1) halves to 0, which would bypass read_metadata's
// earlier check_for_overflow (run before this halving) and reach the pixel readers,
// which assume height >= 1 when computing `self.height - 1`.
if self.height == 0 {
    return Err(DecoderError::InvalidHeight.into());
}
Ok(())
```

Verified against two inputs, not just the one triggering case: the odd-doubled-height (1) input
now returns `Format error decoding Bmp: Invalid height` instead of panicking, and a
valid even-doubled-height (2, real height 1) control input still decodes correctly as a 1x1 image
both before and after the fix - confirming the fix doesn't overreject legitimate ICO-embedded BMPs.

- Project test suite (`cargo test --lib`, run in the main project's own working tree): **331
  passed, 0 failed** before the patch -> **331 passed, 0 failed** after the patch (identical pass
  count, no new failures, no regressions).

`patch.diff` contains this fix as a diff against `base_commit`
(`44ce9226c541dd2b11897b9ef07156a28871cdb7`). `just test` in this candidate's own folder was run
end-to-end against a fresh clone at that commit and confirmed the same before/after behavior
(panic -> clean error), see `output.txt`.

## What's not done / open questions

- Only the RGB24 pixel-reader path (`read_full_byte_pixel_data`) was exercised directly by the
  repro; the candidate description named a different reader (`read_palettized_pixel_data`, line
  1870) with the identical `(self.height - 1).try_into::<u32>().unwrap()` pattern - not
  independently re-verified with a palettized (1/4/8bpp) embedded BMP, since the fix sits upstream
  of all pixel readers (the halving happens once, before any reader is selected) and so covers
  every reader identically; the RGB24 case is representative of the whole class, not a narrower
  fix that happens to miss the others.
- Did not check whether an odd doubled height combined with a *negative* real height (top-down
  BMPs, negative height convention) has any interaction with this fix - the crafted repro only
  covers a positive doubled height.
- Did not independently check the sibling `Os2V2`/`Core` BMP header paths inside an ICO (only the
  standard `BITMAPINFOHEADER`/RGB24 path was exercised) - they funnel through the same
  `read_metadata()` -> `read_metadata_in_ico_format()` halving, so the same fix should cover them,
  but this wasn't separately re-verified with a crafted OS/2-header ICO.
- Upstream PR #3041 (https://github.com/image-rs/image/pull/3041, open/not merged as of this
  session, 2026-08-27) proposes the same fix approach (reject the zero height right after halving);
  this session's patch was derived independently from reading the code, not copied from that PR,
  and was verified against this repo's own test suite and a standalone repro as described above.
