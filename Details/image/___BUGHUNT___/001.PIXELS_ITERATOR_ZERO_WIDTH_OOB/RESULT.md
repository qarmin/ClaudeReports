# Pixels iterator OOB panic on zero-width images

`GenericImageView::Pixels::next()` (`src/images/generic_image.rs`, ~lines 156-172) checks
`self.x >= self.width` before checking `self.y >= self.height`. For an image with `width == 0`
and `height > 0`, this is called out in passing in the description of open upstream PR #3007
("I also found a bug in GenericImageView::Pixels ... didn't specify its stop condition correctly,
which made OOB panics possible") but is not tracked under its own issue/PR.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug build | `ImageBuffer::<Rgba<u8>, Vec<u8>>::new(0, 5)`, iterate via `GenericImageView::pixels(&buf)`, `cargo run --example` (debug profile, debug-assertions on) | Panics: `Image index (0, 1) out of bounds (0, 5)` at `src/images/buffer.rs:1302:15`. Repeated 3/3 identical runs. |
| Standalone `repro/` (release profile) against the crate's own public API, via `just test` end-to-end (real `git clone` of `repo_url` @ `base_commit`, real cargo build) | `cargo run --release` in `repro/` | Same panic before the patch; `did not reproduce - iterated 0 pixels without panicking` after `patch.diff` is applied. |
| Boundary sweep of the fix across neighboring cases | `(0,5)`, `(5,0)`, `(0,0)`, `(3,2)` via a throwaway `examples/` binary (deleted before end of session, output captured in `output.txt`) | All four produce the correct pixel count/order after the fix; the normal `(3,2)` case is unaffected. |

Only one approach round was needed (soft cap of 3 not exceeded) - the bug reproduces immediately
and deterministically via direct boundary-value construction, no fuzzing/miri/sanitizer needed for
this class of bug (a plain, deterministic two-counter iterator logic error, not `unsafe`/UB).

## Status

**Confirmed.** Deterministic, reproduces 3/3 debug runs and again via the standalone `repro/` in
release mode, driven purely by public-API calls (`ImageBuffer::new` + `GenericImageView::pixels`).
No randomness/timing/memory-layout dependence involved.

**Fully reproducible: Yes.** The repro calls the real public entry point
(`GenericImageView::pixels`) on a real `ImageBuffer` built through the crate's own public
constructor - there is no second participant/peer/wire input involved in this finding (it's a
pure logic bug in iterator bookkeeping triggered by a caller-supplied dimension, not a
reaction to another participant's data), so CONFIRMATION.md section 11's "real second
participant" requirement does not apply here.

## Class

crash

## Root cause

`Pixels::next()` advances row-by-row using two separate conditions:

```rust
fn next(&mut self) -> Option<(u32, u32, I::Pixel)> {
    if self.x >= self.width {
        self.x = 0;
        self.y += 1;
    }
    if self.y >= self.height {
        None
    } else {
        let pixel = self.image.get_pixel(self.x, self.y);
        ...
```

The first `if` is meant to detect "we just finished a row, move to the next one" - it implicitly
assumes every row has at least one column, i.e. `width >= 1`. When `width == 0`, `self.x (0) >=
self.width (0)` is true on the very *first* call, before any pixel of "row 0" was ever produced.
This unconditionally increments `y` to 1 and then falls through to the second `if`: since
`height > 0`, `self.y (1) >= self.height` is false, so the code proceeds to call
`self.image.get_pixel(0, 1)` - a coordinate that is out of bounds for every row once `width == 0`
(there are zero valid columns at any row). `ImageBuffer::get_pixel` panics on out-of-bounds
access (`src/images/buffer.rs:1302`), so the very first call to `.next()` panics instead of
immediately returning `None` (the correct behavior: an image with zero columns has zero pixels
regardless of height).

`width == 0, height == 0` and `width > 0, height == 0` do not hit this: in both cases the second
`if` (`self.y >= self.height`) is true immediately (`0 >= 0`), which correctly returns `None`
before any `get_pixel` call. Only the `width == 0 && height > 0` combination is affected.

## Introduced by

d0b2b7b "Implement immutable and mutable pixel iterators. Implement an image trait to allow generic access to images. Provide ImageBuf for directly manipulating images. Provide DynamicImage for images whose pixel type is determined at runtime. Improve the Pixel trait Rearrange library(again)" (2014-07-20)

## The fix

Added an explicit early-out for `width == 0` at the top of `next()`, before the row-wrap logic
that assumes a nonempty row:

```rust
fn next(&mut self) -> Option<(u32, u32, I::Pixel)> {
    if self.width == 0 {
        return None;
    }
    if self.x >= self.width {
        ...
```

This is sufficient because a zero-width image has zero pixels for *any* height, so returning
`None` unconditionally as soon as `width == 0` is checked exactly once per `next()` call is both
correct and cheap (no behavior change for `width > 0`, since the added branch is never taken in
that case). Verified against the four boundary combinations `(0,5)`, `(5,0)`, `(0,0)`, `(3,2)` -
see "What was tried" above and `output.txt` - not just the one triggering input.

- Project test suite (`cargo test --lib`): **331 passed, 0 failed** before the patch -> **331
  passed, 0 failed** after the patch (identical pass count, no new failures, no regressions).

`patch.diff` contains this fix as a diff against `base_commit` (`44ce9226c541dd2b11897b9ef07156a28871cdb7`).

## What's not done / open questions

- Checked for an analogous `PixelsMut`-style iterator on the mutable side: there isn't one - the
  `GenericImage`/`GenericImageView` trait pair only defines the immutable `Pixels` iterator shown
  here. `ImageBuffer`'s own `pixels_mut()`/`EnumeratePixelsMut` (in `src/images/buffer.rs`) are a
  structurally different implementation (built directly over `chunks_exact_mut` on the backing
  slice, not an `(x, y)` counter pair), and a zero-width buffer's slice is simply empty there, so
  they don't share this bug class - not re-verified with a dedicated repro this session, but the
  code shape rules it out by inspection.
- Open upstream PR #3007 (not merged as of this session) fixes this differently, as part of a
  broader change introducing a non-zero image-size type; this session's fix is narrower - a
  single guard clause - and does not attempt that larger refactor.
- Did not check whether any other `GenericImageView`-implementing type in the crate (e.g.
  `SubImage`) could reach the same `Pixels::next()` code path with a zero-width *view* into a
  non-zero-width backing image (e.g. `view.pixels()` on a `Rect` of width 0) - the fix is in the
  shared iterator itself so it should cover that case too, but it wasn't exercised with a
  dedicated boundary test this session.
