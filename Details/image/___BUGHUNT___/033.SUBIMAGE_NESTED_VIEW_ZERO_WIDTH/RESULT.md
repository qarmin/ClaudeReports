# SubImage::view() inherent method (nested subimage-of-subimage) zero-width Rect through Pixels::next()

Follow-up to candidate 032, closing the specific gap that candidate's own "What's not done" section
flagged: `SubImage::view()`'s *inherent* method (`src/images/sub_image.rs:140`, for constructing a
sub-view of an *existing* `SubImage` - i.e. nesting a view inside a view - distinct from the
`GenericImageView::view()` trait method candidate 032 exercised directly on a plain `ImageBuffer`)
was reasoned about by code inspection only, never exercised with a dedicated repro. It calls the
same `Rect::assert_in_bounds_of` / `SubImage::new` construction path as the trait method, so it was
suspected to share the identical defect.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug build, in the main project's own working tree | 5x5 `ImageBuffer<Rgba<u8>, Vec<u8>>` -> level-1 `SubImage` via `GenericImageView::view()` (full-size, 5x5) -> level-2 `SubImage` via the level-1 `SubImage`'s own **inherent** `view()` method (`src/images/sub_image.rs:140`), with a zero-width `Rect` at xoffset=0 (Case A) and xoffset=5=backing width (Case B) | Rect construction succeeds silently in both cases (no assertion panic) - `Rect::is_in_bounds` only checks `x + width <= image_width`, trivially true for `width == 0`. Calling `.pixels()` on the resulting nested `SubImage`: Case A yields 4 phantom pixels (should be 0), Case B panics `Image index (5, 1) out of bounds (5, 5)` at `src/images/buffer.rs:1302:15`. Repeated 3/3 identical runs for both cases (see `output.txt`) |
| Standalone `repro/` (release profile) against the crate's own public API, run via `just test` end-to-end (real `git clone` of `repo_url` @ `base_commit`, real cargo build, actually executed - not just read) | `cargo run --release` in `repro/`, before and after `patch.diff` | Same two outcomes (Case A phantom pixels, Case B panic) before the patch; both "did not reproduce" after `patch.diff` is applied - see `output.txt` |

Only one approach round was needed (soft cap of 3 not exceeded) - the bug reproduces immediately
and deterministically via direct boundary-value construction through the public `SubImage::view()`
inherent method, no fuzzing/miri/sanitizer needed for this class of bug (identical iterator
bookkeeping error to candidates 001 and 032, reached through a newly-verified entry point).

## Status

**Confirmed.** Deterministic, reproduces 3/3 debug runs in the main project's own working tree
(both Case A and Case B) and again via the standalone `repro/` in release mode via `just test`,
driven purely by public-API calls (`ImageBuffer::from_fn` + `GenericImageView::view` to build a
level-1 `SubImage`, then that `SubImage`'s own inherent `.view()` method to nest a level-2
`SubImage`, then `.pixels()`). No randomness/timing/memory-layout dependence involved.

**Fully reproducible: Yes.** The repro calls the real public entry points end-to-end: the crate's
own public `ImageBuffer` constructor, `GenericImageView::view()`, `SubImage`'s own inherent
`view()` method (the exact method this candidate targets, `src/images/sub_image.rs:140`), and the
resulting nested `SubImage`'s `.pixels()` (reached via `Deref` coercion to `SubImageInner`'s
`GenericImageView` impl). There is no second participant/peer/wire input involved in this finding -
it is a pure logic bug in shared iterator bookkeeping, triggered by a caller-supplied `Rect`, not a
reaction to another participant's data - so `CONFIRMATION.md` section 11's "real second
participant" requirement does not apply here.

## Class

crash

(Also has a non-crashing sibling manifestation - see "Root cause" below; classified as `crash`
because that is the more severe of the two observable outcomes of the same defect.)

## Root cause

Identical underlying defect to candidates 001 and 032 (`Pixels::next()`'s row-wrap logic assumes
every row has at least one column when `width == 0`), now verified through a third entry point.

`SubImage::view()`'s inherent method (`src/images/sub_image.rs:140-145`):

```rust
pub fn view(&self, mut rect: Rect) -> SubImage<&I::Target> {
    rect.assert_in_bounds_of(&self.inner);
    rect.x = self.inner.xoffset.saturating_add(rect.x);
    rect.y = self.inner.yoffset.saturating_add(rect.y);
    SubImage::new(&*self.inner.image, rect)
}
```

calls `rect.assert_in_bounds_of(&self.inner)`, which checks the *relative* rect against the
*current* `SubImage`'s own dimensions (`self.inner.xstride`/`self.inner.ystride`) via
`Rect::is_in_bounds` (`src/math/rect.rs`) - this only checks `x + width <= width` and
`y + height <= height`, both trivially satisfied by any zero-width rect at any valid x. It then
translates the rect's offset into the coordinate space of the *original backing image*
(`self.inner.xoffset.saturating_add(rect.x)`) and constructs a brand-new `SubImage` directly against
`&*self.inner.image` (the backing image, not the intermediate level-1 `SubImage`) via
`SubImage::new`. Nothing in this path constrains or special-cases `width == 0`, so it constructs a
`SubImageInner` with `xstride == 0` exactly as the single-level trait-method path did.

Calling `.pixels()` on this nested `SubImage` resolves to the same `GenericImageView::pixels()`
default method, producing `Pixels { width: 0, height: ystride, .. }`. `Pixels::next()`'s first `if`
(`self.x >= self.width`) is true on the very first call when `width == 0`, unconditionally
advancing `y` to 1 and falling into `self.image.get_pixel(0, 1)` on the underlying
`SubImageInner`, which forwards to `get_pixel(x + xoffset, y + yoffset)` on the backing image.
Whether that backing-image call is in-bounds depends entirely on the nested rect's translated
`xoffset`:

- **Case A** (nested rect xoffset = 0, translated xoffset stays 0): `get_pixel(0, y)` stays
  in-bounds of the 5x5 backing image for every row up to its height, so no panic - the iterator
  silently yields 4 phantom pixels from column 0 of the backing image instead of the correct
  answer of zero.
- **Case B** (nested rect xoffset = 5 = backing width, translated xoffset stays 5): `get_pixel(5,
  y)` is out of bounds of the 5x5 backing image (column 5 doesn't exist), so `ImageBuffer::get_pixel`
  panics.

This confirms candidate 032's suspicion by inspection: the inherent `SubImage::view()` method
shares the exact same defect as the trait `GenericImageView::view()` method, because both paths
ultimately construct a `SubImageInner`/`Pixels` pair through the same unguarded machinery, and the
actual defect lives entirely in the shared `Pixels::next()` iterator, not in either `view()`
constructor itself.

## Introduced by

d0b2b7ba "Implement immutable and mutable pixel iterators. Implement an image trait to allow generic access to images. Provide ImageBuf for directly manipulating images. Provide DynamicImage for images whose pixel type is determined at runtime. Improve the Pixel trait Rearrange library(again)" (2014-07-20)

## The fix

Same fix as candidates 001 and 032 (re-verified here to also cover the nested-`SubImage` entry
point): an explicit early-out for `width == 0` at the top of `Pixels::next()`
(`src/images/generic_image.rs`), before the row-wrap logic that assumes a nonempty row.

```rust
fn next(&mut self) -> Option<(u32, u32, I::Pixel)> {
    if self.width == 0 {
        return None;
    }
    if self.x >= self.width {
        ...
```

Verified this single guard clause (identical to candidates 001 and 032's `patch.diff`, no
additional `SubImage`-specific change needed) fixes both Case A (now yields 0 pixels, no phantom
data) and Case B (now yields 0 pixels, no panic) through the nested-`SubImage` entry point - see
`output.txt`.

- Project test suite (`cargo test --lib`, run against the main project's own working tree):
  **331 passed, 0 failed** before the patch -> **331 passed, 0 failed** after the patch (identical
  pass count, no new failures, no regressions).

`patch.diff` contains this fix as a diff against `base_commit`
(`44ce9226c541dd2b11897b9ef07156a28871cdb7`) - identical in content to candidates 001 and 032's
`patch.diff`, included here in full so this folder stays self-contained and independently postable.

## What's not done / open questions

- This closes the last explicitly-flagged gap from candidate 032 (the inherent `SubImage::view()`
  method). The mutable counterpart, `SubImage::sub_image()` (`src/images/sub_image.rs:161`), takes
  the identical `rect.assert_in_bounds_of` / offset-translation path and was not exercised this
  session - it doesn't reach `Pixels::next()` directly (no iterator involved in constructing a
  mutable view), but a zero-width mutable `SubImage` it produces could still misbehave if later
  read through `GenericImageView`/`.pixels()`; worth a quick dedicated check in a future session.
  Added to `IDEAS.md`.
- As with candidate 032, this only exercises `SubImage`/`SubImageInner`, the crate's one
  `GenericImageView` wrapper type at `base_commit`. No new wrapper types were introduced between
  candidate 032 and this session.
