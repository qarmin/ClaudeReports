# `Rect::new_with_points`'s hard `assert!(p1.x <= p2.x)`/`assert!(p1.y <= p2.y)` panics on a DNG whose `ActiveArea` tag is axis-inverted

`rawler::imgop::Rect::new_with_points` (`imgop/mod.rs:129-138`) asserts `p1.x <= p2.x` and
`p1.y <= p2.y` before computing the rectangle's width/height. `Rect::new_with_dng` (called from
`DngDecoder::get_active_area` with a DNG's own, completely unvalidated `ActiveArea` tag: 4 raw
`usize` values read straight off the file, in `[top, left, bottom, right]` order) builds `p1 =
(left, top)` and `p2 = (right, bottom)` and passes them straight through. Any DNG whose
`ActiveArea` has `top > bottom` (or `left > right`) - a malformed/fuzzed file, not necessarily an
adversarial one - hits this `assert!` and panics, reached from `raw_metadata` alone (`raw_image`/
demosaic never need to run). This is a separate, earlier mechanism than candidate 007's
`as_ltrb_offsets` underflow in the same file: this `assert!` fires (or doesn't) before
`as_ltrb_offsets` is ever called at all.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `Rect::new_with_points`/`new_with_dng` (`imgop/mod.rs:129-147`) and `DngDecoder::get_active_area`/`get_active_area_borders`/`make_camera` (`decoders/dng.rs:364-380,247`) | - | Confirmed the exact mechanism: `ActiveArea`'s 4 raw values are used with no validation that `top <= bottom`/`left <= right` before being fed to `Rect::new_with_dng`, which hard-`assert!`s on exactly that invariant |
| Manual boundary-value construction: a hand-built, spec-shaped 100x100 DNG with `ActiveArea = [50, 0, 10, 100]` (top=50 > bottom=10), built via rawler's own public low-level `TiffWriter`, opened via the real, public `rawler::get_decoder` + `Decoder::raw_metadata` | debug build, 3 runs; release build, 1 run | Deterministic panic, `assertion failed: p1.y <= p2.y`, 3/3 debug + 1/1 release (this is a hard `assert!`, not gated by debug-assertions/overflow-checks) |
| Real, unmodified crash file downloaded from GitHub issue #573's own attachment (`compressed.zip` -> `DJI-MavicAir2-Shotkit-2_IDX_0_RAND_1652207307361082056597831_minimized_545.DNG`, a fuzzer-minimized real-world-derived DNG), opened via the same real `rawler::get_decoder` + `Decoder::raw_metadata` entry point | debug build, 3 runs; release build, 1 run | Deterministic panic, **byte-for-byte identical panic message and call-stack shape** (`Rect::new_with_points` -> `Rect::new_with_dng` -> `DngDecoder::get_active_area` -> `DngDecoder::make_camera` -> `raw_metadata`) to the backtrace pasted into the real issue report itself, 3/3 debug + 1/1 release |
| Second hand-built boundary variant (x-axis inverted: `ActiveArea = [0, 80, 90, 10]`, left=80 > right=10) to vary the input within the same bug class (CONFIRMATION.md section 7) and to isolate this candidate's own fix from candidate 007's separate bug | debug build, 3 runs; release build, 1 run | Deterministic panic, `assertion failed: p1.x <= p2.x` (the sibling assert), 3/3 debug + 1/1 release; after the fix, returns `Ok` cleanly with no further panic of any kind (does not chain into candidate 007's bug, unlike the real crash file) |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after |

Only one approach/tool class was needed (manual boundary-value construction plus the real
attached crash file) - it produced an immediate, fully deterministic, byte-for-byte match to a
real, independently-filed bug report on the first attempt, so the soft cap of 3 escalating
tools/approaches was not needed.

## Status

**Confirmed**, and independently corroborated by a real, still-open, third-party bug report on
the same file/line/call-chain (GitHub issue #573, filed by an automated fuzzer bot with an
attached real crash file). Reproduces 3/3 debug runs + 1/1 release run for both the real crash
file and two independently hand-built boundary variants (top>bottom, left>right), fully
deterministic. Unlike candidate 007's underflow bug, this `assert!` is a hard assertion (not
`debug_assert!`/checked-arithmetic), so it panics identically in **both** debug and release
builds.

**Fully reproducible**: **Yes**. `repro/` embeds the real, unmodified crash file downloaded from
GitHub issue #573's own attachment and calls the real, public `rawler::get_decoder` +
`Decoder::raw_metadata` - the exact same two-call sequence the issue's own repro harness
(`get_raw_file`) uses. Single-participant case (one external file artifact parsed by its real
public entry point) - no second participant to fake, per `CONFIRMATION.md` section 11.

## Class

`crash` (hard `assert!`, reproduces in both debug and release builds).

## Root cause

`Rect::new_with_points` (`imgop/mod.rs:129-138`, before this candidate's fix):

```rust
pub fn new_with_points(p1: Point, p2: Point) -> Self {
  assert!(p1.x <= p2.x);
  assert!(p1.y <= p2.y);
  Self {
    p: p1,
    d: Dim2 { w: p2.x - p1.x, h: p2.y - p1.y },
  }
}
```

`Rect::new_with_dng` (`imgop/mod.rs:146-148`) is the only caller reached from DNG decoding, and it
builds `p1`/`p2` directly from a DNG's own `ActiveArea` tag with **no cross-check that top <=
bottom or left <= right** - `DngDecoder::get_active_area_borders` (`decoders/dng.rs:364-370`)
reads the 4 raw tag values and passes them straight through unvalidated. A crafted, corrupted, or
merely internally-inconsistent DNG whose `ActiveArea` has its top/bottom or left/right edges
swapped hits the `assert!` and panics unconditionally - reached from `raw_metadata` alone, the
very first metadata-reading step for any DNG file, before `raw_image`/demosaic ever run.

This is a distinct, earlier mechanism from candidate 007's bug in the same struct: candidate 007's
`as_ltrb_offsets`/`as_tlbr_offsets` underflow only fires once a `Rect` has already been
successfully constructed and its right/bottom edge exceeds the image's own `width`/`height`. This
candidate's `assert!` fires (or doesn't) strictly earlier, inside the constructor itself, and does
not depend on the image's `width`/`height` at all - only on the internal consistency of the
`ActiveArea` tag's own 4 values. The real issue #573 crash file demonstrates both are independently
reachable on the same file: once this candidate's own `assert!` stops blocking it, the same file
goes on to panic on candidate 007's separate, still-unfixed underflow one call deeper (debug build
only) - see `output.txt`.

## Introduced by

0977872e70 "Add missing CR2 and CRW cameras and testdata" (2022-06-12) - `git show` confirms both
`assert!` lines were added fresh by this commit to a `new_with_points` that previously had no
validation at all (so this is not a regression from working code, but the point where the
now-buggy validation was first introduced).

## The fix

Removed both `assert!`s and changed the width/height computation to `p2.x.saturating_sub(p1.x)`/
`p2.y.saturating_sub(p1.y)` - the same "clamp instead of panic" shape already used for the sibling
functions `as_ltrb_offsets`/`as_tlbr_offsets` in candidate 007's fix in this same struct. An
inverted rectangle (either axis) now produces a `Rect` with zero width and/or zero height on the
offending axis instead of panicking - a reasonable interpretation ("the two edges given are
already inverted/coincident, so there is no area on that axis") consistent with every other
`Rect` constructor in this file already being tolerant of malformed file input after candidate
007's fix.

Verified against **two independent inputs in the same bug class** (CONFIRMATION.md section 7):
- The real issue #573 crash file no longer panics on this candidate's `assert!`; `raw_metadata`
  proceeds past `new_with_points`/`new_with_dng` cleanly (it then panics on the **separate**,
  still-unfixed candidate 007 bug in debug builds only - expected and documented in `output.txt`,
  not a regression from this fix).
- A second, independently hand-built boundary variant (x-axis inverted, `ActiveArea` fitting
  within the image once clamped) returns a fully clean `Ok` after the fix, with **no** panic of
  any kind, in both debug and release builds - demonstrating this candidate's own fix is complete
  and correct on its own terms, isolated from candidate 007's separate bug.

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no existing test exercises an axis-inverted `ActiveArea`, so
  this fix is additive coverage, not a regression risk.

## What's not done / open questions

- `Rect::new_with_points` is also called with file-derived, unvalidated points from several other
  decoders (`decoders/cr2.rs:614`, `decoders/cr3.rs:341-397`, `decoders/dng.rs:398` for
  `MaskedAreas`, `rawimage.rs:348-441` for black areas) - none of these were independently
  fuzzed/boundary-tested this session for the same axis-inversion class; the fix in
  `new_with_points` itself (rather than only at the DNG call site) covers all of them by
  construction, but none of the *other* call sites' own downstream behavior on a now-zero-sized
  `Rect` (rather than a panic) was traced through.
- `Rect::adapt` (`imgop/mod.rs:214-218`, unrelated to this candidate) has its own set of four hard
  `assert!`s (`self.p.x >= master.p.x`, `self.d.w <= master.d.w`, etc.) - the same shape of bug,
  not investigated this session; worth a future candidate.
- `Rect::new_with_borders` (`imgop/mod.rs:143-145`) still does a plain, unchecked
  `dim.w - borders[2]` / `dim.h - borders[3]` subtraction *before* calling `new_with_points` - if
  `borders[2] > dim.w` (or `borders[3] > dim.h`), this underflows on its own, one level higher than
  anything this candidate's fix touches. Not confirmed reachable with attacker-controlled
  `borders` this session; noted back into `../IDEAS.md`.
- The real issue #573 crash file's *second* attached variant ("second file with assertion on `x`")
  was not separately downloaded/tried this session - the first attached file already reproduces
  the `p1.y <= p2.y` assert byte-for-byte, and this candidate's own hand-built x-axis variant
  independently exercises the sibling `p1.x <= p2.x` assert, so the second file was judged
  redundant for confirmation purposes.
