# SubImage::sub_image() (mutable inherent method) zero-width Rect through Pixels::next()

Follow-up to candidates 001/032/033: `SubImage::sub_image()` (`src/images/sub_image.rs:161`), the
*mutable* inherent counterpart of `SubImage::view()` that candidates 032/033 already confirmed,
follows the identical `rect.assert_in_bounds_of` / offset-translation construction path but had
never been exercised directly. It looked suspicious because it builds a `SubImageInner` the exact
same unguarded way, and a zero-width mutable `SubImage` it produces can still be read back through
`GenericImageView`/`.pixels()` afterward.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug build, in the main project's own working tree (temporary scratch `examples/scratch_subimage_mut.rs`, removed after use) | 5x5 `ImageBuffer<Rgb<u8>, Vec<u8>>` -> level-1 mutable `SubImage` via `GenericImage::sub_image` (full-size, 5x5) -> level-2 mutable `SubImage` via the level-1 `SubImage`'s own **inherent** `sub_image()` method (`src/images/sub_image.rs:161`), with a zero-width `Rect` at xoffset=0 (Case A) and xoffset=5=backing width (Case B) | Rect construction succeeds silently in both cases (no assertion panic) - `Rect::is_in_bounds` only checks `x + width <= image_width`, trivially true for `width == 0`. Reading the resulting zero-width mutable `SubImage` via `.pixels()` (through `Deref` to `SubImageInner`'s `GenericImageView` impl): Case A yields 4 phantom pixels (should be 0), Case B panics `Image index (5, 1) out of bounds (5, 5)` at `src/images/buffer.rs:1302:15` |
| Standalone `repro/` (release profile) against the crate's own public API, first against the local working tree, then repeated 3/3 identical runs | `cargo run --release` in `repro/`, 3 consecutive runs | Identical outcome all 3 runs: Case A 4 phantom pixels, Case B panic - see `output.txt` |
| Standalone `repro/` end-to-end via `just test` (real `git clone` of `repo_url` @ `base_commit`, real cargo build, actually executed) | `cargo run --release` in `repro/`, before and after `patch.diff` applied to the fresh clone | Same two outcomes (Case A phantom pixels, Case B panic) before the patch; both "did not reproduce" after `patch.diff` is applied - see `output.txt` |

Only one approach round was needed (soft cap of 3 not exceeded) - the bug reproduces immediately
and deterministically via direct boundary-value construction through the public
`SubImage::sub_image()` inherent method, no fuzzing/miri/sanitizer needed for this class of bug
(identical iterator bookkeeping error to candidates 001/032/033, reached through a previously
unexercised mutable entry point).

## Status

**Confirmed.** Deterministic, reproduces 3/3 debug/release runs in the main project's own working
tree (both Case A and Case B) and again via the standalone `repro/` in release mode via `just test`
against a fresh clone. No randomness/timing/memory-layout dependence involved.

**Fully reproducible: Yes.** The repro calls the real public entry points end-to-end: the crate's
own public `ImageBuffer` constructor, `GenericImage::sub_image()` (trait method, to build a
level-1 mutable `SubImage`), that `SubImage`'s own inherent `sub_image()` method (the exact method
this candidate targets, `src/images/sub_image.rs:161`, to build a nested level-2 mutable
`SubImage`), and the resulting `SubImage`'s `.pixels()` (reached via `Deref` coercion to
`SubImageInner`'s `GenericImageView` impl). There is no second participant/peer/wire input
involved in this finding - it is a pure logic bug in shared iterator bookkeeping, triggered by a
caller-supplied `Rect`, not a reaction to another participant's data - so `CONFIRMATION.md` section
11's "real second participant" requirement does not apply here.

## Class

crash

(Also has a non-crashing sibling manifestation - see "Root cause" below; classified as `crash`
because that is the more severe of the two observable outcomes of the same defect.)

## Root cause

Identical underlying defect to candidates 001/032/033 (`Pixels::next()`'s row-wrap logic assumes
every row has at least one column when `width == 0`), now verified through the mutable
`sub_image()` entry point.

`SubImage::sub_image()` (`src/images/sub_image.rs:161-166`):

```rust
pub fn sub_image(&mut self, mut rect: Rect) -> SubImage<&mut I::Target> {
    rect.assert_in_bounds_of(&self.inner);
    rect.x = self.inner.xoffset.saturating_add(rect.x);
    rect.y = self.inner.yoffset.saturating_add(rect.y);
    SubImage::new(&mut *self.inner.image, rect)
}
```

is line-for-line the mutable counterpart of `SubImage::view()` (`src/images/sub_image.rs:140-145`,
confirmed in candidate 033): `rect.assert_in_bounds_of(&self.inner)` only checks the *relative*
rect against the current `SubImage`'s own dimensions via `Rect::is_in_bounds`
(`src/math/rect.rs`), which only verifies `x + width <= width` and `y + height <= height` - both
trivially true for any zero-width rect at any valid `x`. The rect's offset is then translated into
the backing image's coordinate space (`self.inner.xoffset.saturating_add(rect.x)`) and a new
`SubImage` is constructed directly against `&mut *self.inner.image` via `SubImage::new`, with
nothing constraining `width == 0`.

`sub_image()` itself does not touch `Pixels::next()` at construction time (as the candidate
description anticipated - it just builds a mutable `SubImageInner` with `xstride == 0`, no
iterator involved yet). The defect only surfaces once that zero-width mutable `SubImage` is later
read back through `GenericImageView`/`.pixels()` (reached via `Deref` coercion to
`SubImageInner`'s `GenericImageView` impl, available even though the `SubImage` handle itself is
`&mut`). `Pixels::next()`'s first `if` (`self.x >= self.width`) is true on the very first call when
`width == 0`, unconditionally advancing `y` to 1 and falling into `get_pixel(0, 1)` on the
`SubImageInner`, which forwards to `get_pixel(x + xoffset, y + yoffset)` on the backing image:

- **Case A** (xoffset = 0): `get_pixel(0, y)` stays in-bounds of the 5x5 backing image for every
  row, so no panic - the iterator silently yields 4 phantom pixels from column 0 of the backing
  image instead of the correct answer of zero.
- **Case B** (xoffset = 5 = backing width): `get_pixel(5, y)` is out of bounds of the 5x5 backing
  image, so `ImageBuffer::get_pixel` panics.

This confirms the candidate description's suspicion: the mutable `sub_image()` constructor shares
the exact same defect as the read-only `view()` constructor, because both ultimately build a
`SubImageInner`/`Pixels` pair through the same unguarded machinery - the actual defect lives
entirely in the shared `Pixels::next()` iterator, not in either constructor.

`GenericImage::copy_from`'s own bounds-checked path (`src/images/generic_image.rs`, default impl)
was also inspected as the candidate description flagged it as a possible additional trigger: its
fallback loop (`for k in 0..other.height() { for i in 0..other.width() { ... } }`) iterates
`other`'s dimensions directly rather than going through `Pixels`, so a zero-width `other` simply
makes both loops execute zero times - no phantom-pixel or panic behavior was found on that path.
`copy_from`'s bounds check is therefore not part of this defect.

## Introduced by

d0b2b7ba "Implement immutable and mutable pixel iterators. Implement an image trait to allow generic access to images. Provide ImageBuf for directly manipulating images. Provide DynamicImage for images whose pixel type is determined at runtime. Improve the Pixel trait Rearrange library(again)" (2014-07-20)

## The fix

Same fix as candidates 001/032/033 (re-verified here to also cover the mutable `sub_image()` entry
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

Verified this single guard clause (identical to candidates 001/032/033's `patch.diff`, no
additional `SubImage`-specific change needed) fixes both Case A (now yields 0 pixels, no phantom
data) and Case B (now yields 0 pixels, no panic) through the mutable `sub_image()` entry point -
see `output.txt`.

- Project test suite (`cargo test --lib`, run against the main project's own working tree):
  **331 passed, 0 failed** before the patch -> **331 passed, 0 failed** after the patch (identical
  pass count, no new failures, no regressions).

`patch.diff` contains this fix as a diff against `base_commit`
(`44ce9226c541dd2b11897b9ef07156a28871cdb7`) - identical in content to candidates 001/032/033's
fix, since it is the same shared iterator being patched once for every entry point that reaches it.

## What's not done / open questions

- The trait-level `GenericImage::sub_image()` (`src/images/generic_image.rs:328`, the default
  trait method used directly on an `ImageBuffer` rather than `SubImage`'s own inherent method) was
  used in this candidate's repro only to build a full-size level-1 view, never itself given a
  zero-width `Rect` directly. It follows the exact same `assert_in_bounds_of`/`SubImage::new`
  pattern, so it very likely shares the same defect, but this was not exercised with a dedicated
  zero-width repro this session - added to `IDEAS.md` as a follow-up.
- `try_view()`'s mutable counterpart (there does not appear to be one - `GenericImage` has no
  `try_sub_image()`) was checked for existence but not found; not applicable.
- As with prior candidates in this family, only `Pixels`/immutable iteration was exercised. Whether
  any mutable-iteration equivalent (if one is ever added to the crate) would need the same guard is
  not applicable today since no such iterator currently exists.
