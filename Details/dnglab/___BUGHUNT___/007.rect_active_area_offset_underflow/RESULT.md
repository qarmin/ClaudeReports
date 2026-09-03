# DNG `ActiveArea` larger than the image panics in `Rect::as_ltrb_offsets` (debug builds)

`rawler::imgop::Rect::as_ltrb_offsets`/`as_tlbr_offsets` compute `width - (self.p.x + self.d.w)`
and `height - (self.p.y + self.d.h)` with no bounds check - `HEURISTICS.md`'s "arithmetic on
sizes/lengths/indices derived from input" pattern. `DngDecoder::get_active_area`
(`decoders/dng.rs:374-380`) builds a `Rect` directly from a DNG's own `ActiveArea` tag (4
attacker-controlled `usize` values, read with no validation) and immediately calls
`as_ltrb_offsets(width, height)` with `width`/`height` from the same file's `ImageWidth`/
`ImageLength` tags - if the `ActiveArea` rectangle's right/bottom edge exceeds the image's own
declared dimensions (a mismatch between two independently file-controlled values, not a size
sanity-checked against anything), the subtraction underflows.

**This candidate reproduces an already-open, independently real-user-reported bug**: GitHub issue
#582 on `dnglab/dnglab` ("Panic `attempt to subtract with overflow` in `src/imgop/mod.rs`", filed
by an automated fuzzing bot against a real user-supplied DNG file, still open as of this session's
sync - see `../IDEAS.md`). That issue's own backtrace shows the identical call chain this
candidate's repro reproduces by hand: `Rect::as_ltrb_offsets -> DngDecoder::get_active_area ->
DngDecoder::make_camera -> raw_metadata`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| GitHub issue/PR raw sync (`fetch_issues.py raw-sync`) + reading through the synced open issues for bug-hunting relevance | ~59 open issues, 19 open PRs synced | Found several still-open, real-user-reported crash issues matching this session's own bug classes almost exactly - see `../IDEAS.md` for the full list; issue #582 stood out as reproducible from a cheap, well-understood mechanism (`Rect` arithmetic, same file as candidate 001) reachable from `raw_metadata` alone (no `raw_image`/demosaic needed) |
| Code reading of `Rect::as_ltrb_offsets`/`DngDecoder::get_active_area` (`imgop/mod.rs:164-172`, `decoders/dng.rs:374-380`) | - | Confirmed the exact mechanism: an `ActiveArea` tag whose bottom-right corner exceeds `ImageWidth`/`ImageLength` underflows the subtraction |
| Standalone repro: a hand-built, spec-shaped DNG (100x100, uncompressed, RGGB) with `ActiveArea = [0, 0, height+500, width+500]` - deliberately larger than the image - built via rawler's own public low-level `TiffWriter`, opened via the real, public `rawler::get_decoder` + `Decoder::raw_metadata` (the exact call chain issue #582's own repro harness uses) | debug build, 3 runs; release build, 1 run | Deterministic panic, 3/3 debug runs, **identical panic location and call stack** to issue #582's real-world report; no panic in release (see Status) |
| `cargo test -p rawler --lib` before and after the fix | debug build, 66 tests | 66 passed / 0 failed, both before and after |

## Status

**Confirmed**, and independently corroborated by a real, still-open, third-party bug report on the
same file/line/call-chain (issue #582). Reproduces 3/3 debug runs, fully deterministic. Debug-build-
only, same as candidate 006: in release (`overflow-checks = false`), the subtraction silently wraps
to a large `usize` instead of panicking, and `raw_metadata` returns `Ok` with that wrapped value
stored as part of the camera's active-area data.

**Fully reproducible**: **Yes**. `repro/` writes a real, spec-shaped DNG file to disk (differing
from a normal file only in its `ActiveArea` tag exceeding the image bounds) and calls the real,
public `rawler::get_decoder` + `Decoder::raw_metadata` - the exact same two-call sequence issue
#582's own repro harness uses. Single-participant case, no second participant to fake, per
`CONFIRMATION.md` section 11.

## Class

`crash` (debug build only - see Status).

## Root cause

`Rect::as_ltrb_offsets` / `as_tlbr_offsets` (`imgop/mod.rs:164-172`):

```rust
pub fn as_ltrb_offsets(&self, width: usize, height: usize) -> [usize; 4] {
  [self.p.x, self.p.y, width - (self.p.x + self.d.w), height - (self.p.y + self.d.h)]
}
```

Called from `DngDecoder::get_active_area` (`decoders/dng.rs:374-380`) with a `Rect` built directly
from the DNG's own `ActiveArea` tag and `width`/`height` from the same file's `ImageWidth`/
`ImageLength` tags - two independently file-controlled values with no cross-validation that one
fits inside the other. A crafted (or simply corrupted/inconsistent) DNG whose `ActiveArea`
rectangle extends past the declared image dimensions makes `self.p.x + self.d.w > width` (or the
`y`/`height` equivalent), underflowing the subtraction.

## Introduced by

416d897 "Code refactoring for more raw formats" (2022-04-10) - `git show` confirms `as_ltrb_offsets`/`as_tlbr_offsets` were added fresh by this commit (not a rename of prior code), already with the unchecked `width - (...)`/`height - (...)` subtraction shape.

## The fix

Changed both subtractions in `as_ltrb_offsets` and `as_tlbr_offsets` to `width.saturating_sub(...)`/
`height.saturating_sub(...)` - the same fix shape used for candidates 001/002 in this workspace.
When the `ActiveArea` genuinely fits inside the image (the normal case) this is identical to the
original expression; when it doesn't, the offset saturates to 0 instead of panicking, which is a
reasonable interpretation ("the active area's edge is already at or past the image edge, so there
is no border on that side") consistent with what these functions' names promise.

Verified: `repro/`'s oversized-`ActiveArea` DNG no longer panics in debug and `raw_metadata`
returns `Ok`, matching what release builds already silently did (now reached deliberately via the
saturating fix instead of accidentally via wraparound).

- Project test suite: 66 passed / 0 failed before -> 66 passed / 0 failed after (`cargo test -p
  rawler --lib`). No new failures; no existing test exercises an `ActiveArea` exceeding the image
  bounds, so this fix is additive coverage, not a regression risk.

## What's not done / open questions

- `self.p.x + self.d.w` (and the `y`/`height` equivalent) is still a plain, unchecked addition -
  for genuinely extreme attacker-controlled `ActiveArea` values (e.g. near `usize::MAX`) this could
  itself overflow before `saturating_sub` ever runs. Not independently confirmed reachable this
  session (would need `ActiveArea` values in the billions for a real image-dimension `usize`,
  implausible for any real camera file, but not impossible for a deliberately adversarial one).
- The release-mode wrapped-to-a-huge-value active-area border was not traced further downstream
  to see whether any later code (crop, black-area computation) misuses that wrapped value in an
  unsound way (e.g. as a slice bound) - candidate 001's own root cause was exactly this shape of
  "release-mode wraparound removes a bound that protects a later unchecked access" in a different
  file, so this is a plausible-but-unconfirmed risk worth a future session's attention.
- Issue #582 was the only one of the several real-user-reported crash issues (see `../IDEAS.md`
  for the full list: #569, #571, #573, #574, #583, #586, #587, plus #570/#572 which are
  OOM/hang-shaped rather than panic-shaped) investigated this session - the others remain open
  backlog items with exact panic locations already extracted from their bot-filed backtraces,
  ready for a future session to pick up directly without re-fuzzing.
