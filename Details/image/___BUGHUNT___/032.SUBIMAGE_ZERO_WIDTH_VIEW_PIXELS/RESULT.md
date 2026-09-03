# SubImage::view() zero-width Rect through Pixels::next() (follow-up to candidate 001)

Candidate 001 fixed `GenericImageView::Pixels::next()`'s zero-width out-of-bounds panic by adding
a `width == 0` early-out guard, but that session's "What's not done" section explicitly flagged
that `SubImage::view()`/`try_view()` constructing a zero-width `Rect` *view* into a non-zero-width
backing image, and then reaching the same `Pixels::next()` path through `SubImageInner`'s own
`GenericImageView` impl, was never exercised with a dedicated boundary repro. This candidate closes
that gap.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug build | 5x5 `ImageBuffer<Rgba<u8>, Vec<u8>>`, `Rect::assert_in_bounds_of` reasoning confirmed `Rect { x: 0, y: 0, width: 0, height: 5 }` and `Rect { x: 5, y: 0, width: 0, height: 5 }` are both trivially in-bounds of a 5x5 image (`is_in_bounds` only checks `x + width <= W` and `y + height <= H`), so `GenericImageView::view()` (not just `try_view()`) accepts a zero-width `Rect` without any assertion firing | Confirmed both `Rect`s construct successfully via the public `view()` API - no assertion panic at construction time, so the bad state does reach the shared `Pixels` iterator |
| `cargo run --example` in the main project's own working tree (debug profile, debug-assertions on), driving `backing.view(rect).pixels()` through `SubImage`'s `Deref`-coerced `GenericImageView` impl | Two cases: xoffset=0 (Case A) and xoffset=backing width (Case B), both zero-width, height=5, into a 5x5 backing image | Case A: **no panic** - silently yields 4 phantom pixels (should be 0). Case B: **panics** - `Image index (5, 1) out of bounds (5, 5)` at `src/images/buffer.rs:1302:15`. Repeated 3/3 identical runs for both cases (see `output.txt`) |
| Standalone `repro/` (release profile) against the crate's own public API, via `just test` end-to-end (real `git clone` of `repo_url` @ `base_commit`, real cargo build) | `cargo run --release` in `repro/` | Same two outcomes (Case A phantom pixels, Case B panic) before the patch; both "did not reproduce" after `patch.diff` is applied |

Only one approach round was needed (soft cap of 3 not exceeded) - the bug reproduces immediately
and deterministically via direct boundary-value construction through the public `SubImage` API, no
fuzzing/miri/sanitizer needed for this class of bug (the same plain iterator-bookkeeping logic
error as candidate 001, reached through a different entry point).

## Status

**Confirmed.** Deterministic, reproduces 3/3 debug runs (both Case A and Case B) and again via the
standalone `repro/` in release mode, driven purely by public-API calls
(`ImageBuffer::from_fn` + `GenericImageView::view` + `.pixels()`). No randomness/timing/memory-
layout dependence involved.

**Fully reproducible: Yes.** The repro calls the real public entry points
(`GenericImageView::view` and the resulting `SubImage`'s `.pixels()`, reached via `Deref`
coercion to `SubImageInner`'s `GenericImageView` impl) on a real `ImageBuffer` built through the
crate's own public constructor. There is no second participant/peer/wire input involved in this
finding - it is a pure logic bug in shared iterator bookkeeping, triggered by a caller-supplied
`Rect`, not a reaction to another participant's data - so `CONFIRMATION.md` section 11's "real
second participant" requirement does not apply here.

## Class

crash

(Also has a non-crashing sibling manifestation - see "Root cause" below; classified as `crash`
because that is the more severe of the two observable outcomes of the same defect.)

## Root cause

Same underlying defect as candidate 001 (`Pixels::next()`'s row-wrap logic assumes every row has
at least one column when `width == 0`), but reached through a previously-unverified entry point
and with two distinct observable failure modes depending on the view's x-offset:

`Rect::is_in_bounds` (`src/math/rect.rs`) only checks `x + width <= image_width` and
`y + height <= image_height` - a zero-width `Rect` at any valid `x` offset trivially satisfies
this (`x + 0 <= image_width` is true whenever `x <= image_width`), so both `GenericImageView::view()`
(via `assert_in_bounds_of`) and `try_view()` (via `test_in_bounds_of`) happily construct a
`SubImage` whose `SubImageInner` has `xstride == 0` and `ystride > 0`. Calling `.pixels()` on that
`SubImage` resolves (via `Deref` to `SubImageInner`) to the same `GenericImageView::pixels()`
default method candidate 001 examined, producing a `Pixels { width: 0, height: ystride, .. }`.

`Pixels::next()`'s first `if` (`self.x >= self.width`) is true on the very first call when
`width == 0`, unconditionally advancing `y` to 1 and falling into `self.image.get_pixel(0, 1)` -
but here `self.image` is `SubImageInner`, whose `get_pixel(x, y)` implementation
(`src/images/sub_image.rs`) forwards to `self.image.get_pixel(x + self.xoffset, y + self.yoffset)`
on the *backing* image. Whether this backing-image call is in-bounds depends entirely on
`xoffset`, which the zero-width-`Rect` guard never constrained:

- **xoffset = 0** (Case A): `get_pixel(0 + 0, y + 0)` stays in-bounds of the backing image for
  every row up to its height, so no panic occurs at all - the iterator silently yields pixels from
  column 0 of the *backing* image (4 phantom pixels in the 5-row repro) instead of the correct
  answer of zero pixels for a zero-width view. This is a **silent wrong-data bug**, strictly worse
  than a panic because nothing signals the caller that anything went wrong.
- **xoffset = backing width** (Case B): `get_pixel(xoffset + 0, y + 0)` is now out of bounds of
  the backing image (column `xoffset` doesn't exist), so `ImageBuffer::get_pixel` panics exactly
  as in candidate 001's direct-`ImageBuffer` repro, just one level of indirection further in.

## Introduced by

d0b2b7ba "Implement immutable and mutable pixel iterators. Implement an image trait to allow generic access to images. ..." (2014-07-20) - the original `Pixels`-style iterator's row-wrap logic never guarded against a zero-width row, and this assumption survived unchanged through later refactors (moves/renames verified via `git log --follow -S`) into today's `Pixels::next()` in `src/images/generic_image.rs`.

## The fix

Same fix as candidate 001 (this session re-verified it also covers this entry point, per
candidate 001's own "What's not done" note that the fix should structurally cover `SubImage` since
it's the same shared iterator, but had never actually been exercised through this path): an
explicit early-out for `width == 0` at the top of `Pixels::next()`, before the row-wrap logic that
assumes a nonempty row.

```rust
fn next(&mut self) -> Option<(u32, u32, I::Pixel)> {
    if self.width == 0 {
        return None;
    }
    if self.x >= self.width {
        ...
```

Verified this single guard clause (identical to candidate 001's `patch.diff`, no additional change
needed) fixes both Case A (now yields 0 pixels, no phantom data) and Case B (now yields 0 pixels,
no panic) - see `output.txt`. No `SubImage`-specific code needed to change; the shared iterator fix
is sufficient because the guard is checked once per `next()` call regardless of which
`GenericImageView` implementation supplies `get_pixel`.

- Project test suite (`cargo test --lib`, run against the main project's own working tree):
  **331 passed, 0 failed** before the patch -> **331 passed, 0 failed** after the patch (identical
  pass count, no new failures, no regressions).

`patch.diff` contains this fix as a diff against `base_commit` (`44ce9226c541dd2b11897b9ef07156a28871cdb7`)
- identical in content to candidate 001's `patch.diff`, included here in full so this folder stays
  self-contained and independently postable.

## What's not done / open questions

- This candidate only exercises `SubImage`/`SubImageInner` (the crate's one `GenericImageView`
  wrapper type). If a future change to the crate adds another `GenericImageView`-implementing
  wrapper/adapter type, it would be worth re-checking whether it can similarly construct a
  zero-width view into a non-zero-width backing image - not applicable today since `SubImage` is
  the only such type at `base_commit`.
- Did not check whether `SubImage::view()`'s own inherent method (`src/images/sub_image.rs:140`,
  distinct from the `GenericImageView::view()` trait method used in this repro, for constructing a
  sub-view of an existing `SubImage` rather than a plain image) has the same reachability - it
  calls the same `Rect::assert_in_bounds_of` / `SubImage::new` construction path, so by inspection
  it very likely shares this exact defect, but wasn't exercised with a dedicated repro this
  session.
