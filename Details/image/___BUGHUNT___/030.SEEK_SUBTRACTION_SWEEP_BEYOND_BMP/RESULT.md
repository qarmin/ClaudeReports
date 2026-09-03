# Seek-position unchecked-subtraction sweep beyond BMP

Follow-up from `IDEAS.md` row 44 (candidate 029's leftover): candidate 029 found and fixed one
unchecked-subtraction-of-two-`Seek`-positions bug in BMP's `read_scanline`
(`end_pos - current_pos`, where `current_pos` derives from an attacker-controlled `data_offset`
field), but never grepped the rest of the codebase for the same shape. This candidate does that
sweep: every place in this repo's own source that calls `.seek(...)`/`.stream_position()` and then
does arithmetic on the resulting position(s).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Exhaustive grep for `Seek`/`stream_position` usage across all of `src/` | `grep -rn "stream_position\|SeekFrom::Start\|SeekFrom::End\|\.seek(" src/` (and re-run scoped per-codec) | Every hit outside `src/codecs/bmp/decoder.rs` and `src/codecs/ico/decoder.rs` is either doc comment text or `src/hooks.rs`'s pass-through `Seek` trait impl (no arithmetic). `src/codecs/tiff.rs` and `src/codecs/png.rs` do not do their own `Seek`-position arithmetic - IFD offset handling / chunk offset handling is entirely inside the external `tiff`/`png` crates, outside this repo's own source tree and this sweep's scope. GIF, WebP, JPEG, HDR, PNM, TGA, AVIF, OpenEXR, QOI, and farbfeld codecs contain no `Seek`/`stream_position` calls at all in this repo. |
| Manual read of every remaining `Seek`-arithmetic site in `src/codecs/bmp/decoder.rs` (14 call sites: header/palette/ICC-profile phase seeks, per-row `file_offset` computation for palettized/16-bit/24-bit/32-bit readers, RLE seek-relative) | Read each site and traced whether the "later minus earlier" subtraction shape from candidate 029 appears anywhere else | Only one instance of the exact underflow shape exists: `read_scanline`'s `end_pos - current_pos` (line ~730, candidate 029's already-documented bug - its verified fix was reverted after that candidate per protocol, so it's still present in the working tree). Every other subtraction in the file (`length - max_length` in `read_palette`, guarded by an `Ordering::Greater` match arm; `header_size - 4`/`remaining - 36` in header parsing, both against header-declared sizes already range-checked earlier in the same function) is either not `Seek`-position-derived or is already guarded before the subtraction executes. The ICC-profile-offset path (`read_icc_profile`) already uses `checked_add`/an explicit `profile_end > stream_len` bound check (added in commit `1eca1bac`, this repo's most recent commit at the time of this sweep) rather than a raw subtraction - not vulnerable to this shape. |
| Manual read of `src/codecs/ico/decoder.rs`'s two `Seek`-arithmetic sites (`with_spec_compliance`'s `reader_offset`, and the embedded-BMP AND-mask handling's `image_end`/`data_end` comparison) | Read `seek_to_start`/`seek_start_with_offset` (`src/utils/mod.rs:127-138`, used by both ICO and the BMP-in-ICO path) and the `data_end >= image_end + mask_length` comparison at `decoder.rs:402` | No subtraction of two `Seek`-derived positions exists in ICO at all - `seek_start_with_offset` only ever *adds* an offset to a base position (via `checked_add`, returning an `io::Error` on overflow rather than seeking), and the AND-mask length check is an addition-based `>=` comparison (`data_end >= image_end + mask_length`), not a subtraction - structurally cannot underflow the way candidate 029's bug did. |

Two escalating approaches were effectively one systematic pass (grep to find every call site, then
read every remaining one by hand) - no fuzzing/miri/ASAN needed since this was purely a
pattern-matching/code-reading sweep, not a suspected-live reproduction of a new bug; the pattern
was exhaustively enumerable by grep and each hit was small enough to read in full.

## Status

**Not reproducible.** No second instance of candidate 029's bug class (unchecked subtraction of
two `Seek`-derived positions where the "current" one is influenced by an attacker-controlled file
offset field) exists anywhere in this repo's own source outside the already-known BMP
`read_scanline` case. Covered: every `.seek(`/`.stream_position()` call site in `src/` (14 in BMP,
2 in ICO, 1 pass-through in `hooks.rs`, 1 in `utils/mod.rs`), confirmed via grep to be exhaustive
(no other codec module contains any `Seek`-position arithmetic at all). TIFF and PNG chunk/IFD
offset handling is delegated entirely to the external `tiff`/`png` crates and is out of this
repo's own source-tree scope - not swept by this candidate (see "What's not done" below).

## Class

n/a (not reproducible - no new bug found)

## Root cause

n/a - this sweep confirms the suspect area (decoders elsewhere in this repo performing
`Seek`-position subtraction) does not exist as a class beyond the single instance already
identified and documented in candidate 029. ICO's own `Seek` arithmetic is exclusively additive
(with an explicit `checked_add`/overflow-`io::Error` guard in `seek_start_with_offset`, and an
addition-based `>=` bound check for the AND-mask case), so it cannot exhibit the same
underflow-on-subtraction failure mode structurally, regardless of what values the offset fields in
an ICO/CUR file contain.

## The fix

Not applicable - no new bug found to fix. The one known instance of this bug class (BMP
`read_scanline`) is already fully documented with a verified fix in
`___BUGHUNT___/029.BMP_READ_SCANLINE_LENIENT_LAST_ROW_UNDERFLOW/`; that fix was not reapplied here
since this candidate found nothing new to combine it with.

## What's not done / open questions

- TIFF IFD-offset handling and PNG chunk-offset handling are both implemented entirely inside the
  external `tiff` and `png` crates this project depends on, not in `image`'s own `src/` tree - this
  sweep did not reach into either dependency's own source. If a future session wants to check
  those crates for the same bug class, it would need to check out and read `tiff`/`png` directly
  (a cross-crate check, structurally the same kind of out-of-scope note candidate 013 left for
  post-validation dimension re-derivation in the same set of upstream crates).
- Row 42's narrower fallback item (general BMP `data_offset`-vs-file-length validation, never
  added beyond the one `read_scanline` consequence candidate 029 patched) remains open and was not
  addressed by this candidate - left in `IDEAS.md`.
