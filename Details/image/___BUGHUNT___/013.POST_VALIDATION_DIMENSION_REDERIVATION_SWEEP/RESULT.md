# Sweep for other post-validation dimension re-derivation sites (generalizing candidate 007)

Generalization of candidate 007 (`BMP_ICO_ODD_HEIGHT_ZERO_PANIC`, Confirmed): that candidate found
`BmpDecoder::read_metadata_in_ico_format` halves `self.height` *after* `check_for_overflow` already
validated the pre-halved value, letting an odd doubled height reintroduce `height == 0`
post-validation. This candidate looks for the same general shape elsewhere in the codebase: any
decoder/codec that (1) validates a width/height against overflow/zero, then (2) derives a *new*
width/height from it afterwards (halved/subtracted/cropped/padded), (3) without re-validating the
derived value, in a context that assumes the derived value stays positive. Selected from the
`IDEAS.md` backlog row left by candidate 007 itself.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Grep sweep #1: every direct reassignment of a decoder's own stored dimension field after initial parse (`self.width =`, `self.height =`, `.height /=`, `.width /=`, `.height >>=`, `.width >>=`, `.height -=`, `.width -=`) across `src/codecs/` | `grep -rn` over the whole `src/codecs/` tree | Only hits: BMP's three header readers assign `self.width`/`self.height` immediately followed by `check_for_overflow(...)` in the same function (no gap for an unvalidated intermediate state), plus the single `self.height /= 2` in `read_metadata_in_ico_format` - the exact site candidate 007 already found and fixed (fix not currently applied in this working tree, since fixes are reverted after each candidate's own verification). No other decoder in the tree reassigns its own stored width/height field after construction. |
| Grep sweep #2: local-variable arithmetic on `width`/`height` that could floor-divide or subtract down to zero/negative (`width / 2`, `height / 2`, `width - `, `height - `, `>> 1`) across `src/codecs/` and `src/images/`, each hit read in context | `grep -rn` + manual read of each match (TGA flip logic, GIF frame compositing, `resize_to_fill`'s crop centering, AVIF `chroma_size`/`chroma_height`, `filter_1d.rs` kernel padding, HDR/BMP row-index subtraction) | No unguarded instance of the bug class. TGA's `self.height / 2` / `self.width / 2` splits an already-known-valid buffer for a flip, not a dimension re-derivation. GIF's `width - effective_left` is pre-clamped via `.min(width)`, so it can't underflow. `resize_to_fill`'s `(iheight - nheight) / 2` is candidate 008's already-investigated/fixed OOM spot, not a zero-dimension issue (both operands are post-`resize_dimensions` positive values by construction of that function). AVIF's chroma-plane sizes use `div_ceil(2)`, not floor division, so a positive input can never round down to 0 (unlike the ICO case's plain `/= 2`, which is exactly why that one was exploitable and these aren't). `filter_1d.rs`'s kernel padding operates on the (odd, compile-time-shaped) kernel size, not on the untrusted image dimensions. HDR/BMP row-index subtractions (`height - 1 - file_row`, `(height - 1)`) all execute only after the containing function's own `height >= 1` precondition already holds (BMP's callers all run post-`check_for_overflow`/post-halving-fix; HDR's `read_image_data` explicitly early-returns on `self.meta.width == 0 \|\| self.meta.height == 0` at `src/codecs/hdr/decoder.rs:326` before ever reaching scanline decode). |
| Manual read of every other format's dimension-parsing path with no arithmetic hit above (QOI, farbfeld, OpenEXR, PNM, TIFF, JPEG, WebP, PNG) for any two-stage "validate, then re-derive" shape not caught by the grep patterns | Read `src/codecs/qoi.rs`, `src/codecs/farbfeld.rs`, `src/codecs/openexr.rs`, `src/codecs/pnm/decoder.rs`, `src/codecs/tiff.rs`, `src/codecs/jpeg/decoder.rs` (JPEG/TIFF/PNG/WebP dimension parsing is delegated entirely to their respective upstream crates - `jpeg-decoder`, `tiff`, `png`, `image-webp` - with `Limits::check_dimensions` called once against the crate-reported dimensions and no further local re-derivation in this repo's own glue code) | No decoder in this repo derives a second width/height from a first, already-checked one, other than the BMP-in-ICO case candidate 007 already found and fixed. |

Only one round of tooling was needed for this candidate (grep sweep + manual read, no debug-build
run or fuzzing) - the suspected bug class is a specific *shape* (validate, then floor-divide/
subtract without re-checking) that a targeted grep across every codec's dimension-handling code can
enumerate exhaustively enough to rule out with reasonable confidence; there was no partial signal
found anywhere that would justify escalating to a fuzz harness or debug-assertions run per
`SKILL.md`'s soft cap guidance.

## Status

**Not reproducible.** No second instance of the pattern (validate a dimension, then derive/adjust a
new dimension from it afterward without re-validating the result, reachable with an untrusted,
already-valid-looking input) was found anywhere else in the codebase as of this session
(2026-08-27, commit `44ce9226c541dd2b11897b9ef07156a28871cdb7`). The one instance that does exist
(BMP-in-ICO height halving) was already found, root-caused, and fixed in candidate 007 - this
candidate is a generalization sweep that found no *additional* occurrence, not a re-check of that
one.

## Root cause

N/A (no live bug found by this candidate). The reason the class doesn't recur elsewhere: every
other dimension-derivation site found either (a) uses ceiling division (`div_ceil`), which cannot
round a positive value down to zero the way the ICO case's plain `/ 2` can; (b) clamps/`.min()`s an
operand before subtracting, so the subtraction cannot underflow; (c) operates on a value that was
already re-validated or explicitly zero-guarded immediately before use (HDR's `read_image_data`);
or (d) is fully delegated to an upstream crate that performs its own dimension parsing/validation
with no local re-derivation in this repo's glue code. The ICO case was exploitable specifically
because it combined a floor division (not ceiling) with a validation call that ran *before* that
division, on the pre-halved value, and nothing re-checked the post-halved result.

## The fix

N/A - no bug found, so no fix to apply. (Candidate 007's own fix, for the one instance that does
exist, is described in `007.BMP_ICO_ODD_HEIGHT_ZERO_PANIC/RESULT.md`/`patch.diff` and was not
reapplied by this candidate, per this skill's rule that only the candidate that found a bug carries
its own fix.)

## What's not done / open questions

- This sweep was scoped to this repo's own source (`src/`) - it did not sweep the internals of
  upstream crates this project delegates dimension parsing to (`jpeg-decoder`, `tiff`, `png`,
  `image-webp`, `dav1d`/`mp4parse` for AVIF, `pic-scale-safe` for resize), several of which are
  already noted as out-of-scope in earlier candidates' backlog entries (e.g. candidate 010's note
  on `pic-scale-safe`). The same bug class could in principle exist inside one of those crates.
- The grep patterns used are pattern-based, not a formal data-flow analysis - a re-derivation
  written in an unusual form (e.g. via a helper function rather than inline arithmetic, or hidden
  behind a trait method) could in principle have been missed. The manual read pass over every
  format without a direct grep hit was intended to catch this, but is not exhaustive proof.
- Did not re-run this candidate's grep sweep after (hypothetically) reapplying candidate 007's fix
  to confirm the fixed tree also greps clean for the same pattern - not necessary, since candidate
  007's own RESULT.md already independently verified that fix.
