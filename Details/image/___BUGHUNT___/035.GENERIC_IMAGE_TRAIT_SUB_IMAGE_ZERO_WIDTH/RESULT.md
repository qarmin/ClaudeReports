# GenericImage::sub_image() (trait default method) zero-width Rect through Pixels::next()

Follow-up to candidates 001/032/033/034: `GenericImage::sub_image()` (`src/images/generic_image.rs:328`),
the trait-default method used directly on a plain `ImageBuffer`/any `GenericImage` (distinct from
`SubImage`'s own inherent `sub_image()`/`view()` methods candidates 032/033/034 already exercised),
follows the identical `rect.assert_in_bounds_of(self)` / `SubImage::new` construction path with
nothing constraining `width == 0`. Flagged as a follow-up during candidate 034 (`IDEAS.md`) as the
last unexercised call site of this defect class in the crate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug build, in the main project's own working tree (temporary scratch `examples/scratch_generic_sub_image.rs`, removed after use) | 5x5 `ImageBuffer<Rgb<u8>, Vec<u8>>` -> single-level `SubImage` via `GenericImage::sub_image()` (the trait default method itself, called directly on the `ImageBuffer`, no `SubImage` nesting), with a zero-width `Rect` at xoffset=0 (Case A) and xoffset=5=backing width (Case B) | Rect construction succeeds silently in both cases (no assertion panic). Reading the resulting zero-width `SubImage` via `.pixels()`: Case A yields 4 phantom pixels (should be 0), Case B panics `Image index (5, 1) out of bounds (5, 5)` at `src/images/buffer.rs:1302:15` |
| Repeated the same scratch binary 3 consecutive runs | `cargo run --example scratch_generic_sub_image`, debug build, 3x | Identical outcome all 3 runs: Case A 4 phantom pixels, Case B panic - see `output.txt` |
| Standalone `repro/` (release profile) against the crate's own public API, first against the local working tree, then via `just test` end-to-end (real `git clone` of `repo_url` @ `base_commit`) before and after `patch.diff` | `cargo run --release` in `repro/` | Same two outcomes (Case A phantom pixels, Case B panic) before the patch; both "did not reproduce" after `patch.diff` is applied - see `output.txt` |

Only one approach round was needed (soft cap of 3 not exceeded) - the bug reproduces immediately
and deterministically on the very first boundary-value construction, no fuzzing/miri/sanitizer
needed for this class of bug (identical iterator bookkeeping error to candidates 001/032/033/034,
now confirmed through the last remaining unexercised entry point: the trait-default method used
directly on a plain `ImageBuffer`, no `SubImage` nesting).

## Status

**Confirmed.** Deterministic, reproduces 3/3 debug runs in the main project's own working tree
(both Case A and Case B) and again via the standalone `repro/` in release mode via `just test`
against a fresh clone. No randomness/timing/memory-layout dependence involved.

**Fully reproducible: Yes.** The repro calls the real public entry points end-to-end: the crate's
own public `ImageBuffer` constructor and `GenericImage::sub_image()` (the trait default method
this candidate targets, `src/images/generic_image.rs:328`, called directly on the `ImageBuffer` -
no `SubImage`-on-`SubImage` nesting, unlike candidate 034), followed by the resulting `SubImage`'s
`.pixels()` (via `GenericImageView`). There is no second participant/peer/wire input involved in
this finding - it is a pure logic bug in shared iterator bookkeeping, triggered by a
caller-supplied `Rect`, not a reaction to another participant's data - so `CONFIRMATION.md` section
11's "real second participant" requirement does not apply here.

## Class

crash

(Also has a non-crashing sibling manifestation - see "Root cause" below; classified as `crash`
because that is the more severe of the two observable outcomes of the same defect.)

## Root cause

Identical underlying defect to candidates 001/032/033/034 (`Pixels::next()`'s row-wrap logic
assumes every row has at least one column when `width == 0`), now verified through the
trait-default `sub_image()` entry point on a plain `ImageBuffer`, without any `SubImage` nesting.

`GenericImage::sub_image()` (`src/images/generic_image.rs:325-331`):

```rust
fn sub_image(&mut self, rect: Rect) -> SubImage<&mut Self>
where
    Self: Sized,
{
    rect.assert_in_bounds_of(self);
    SubImage::new(self, rect)
}
```

`rect.assert_in_bounds_of(self)` only checks `x + width <= image_width` and
`y + height <= image_height` via `Rect::is_in_bounds` (`src/math/rect.rs`) - both trivially true
for any zero-width rect at any valid `x`, including `x == image_width`. `SubImage::new` is then
constructed directly with `xstride == 0`, with nothing constraining `width == 0`. The defect
doesn't surface at construction - it only shows up once the resulting zero-width `SubImage` is
read back through `.pixels()` (`GenericImageView`'s default `pixels()` -> `Pixels::next()`,
`src/images/generic_image.rs`): the first `if` (`self.x >= self.width`) is true on the very first
call when `width == 0`, unconditionally advancing `y` to 1 and falling into `get_pixel(0, 1)`:

- **Case A** (xoffset = 0): `get_pixel(0, y)` stays in-bounds of the 5x5 backing image for every
  row, so no panic - the iterator silently yields 4 phantom pixels from column 0 of the backing
  image instead of the correct answer of zero.
- **Case B** (xoffset = 5 = backing width): `get_pixel(5, y)` is out of bounds of the 5x5 backing
  image, so `ImageBuffer::get_pixel` panics.

This is the fifth and (per candidate 034's own "what's not done" list) last previously-unexercised
call site of this exact defect class: `Pixels`'s zero-width-row assumption is entirely independent
of which constructor built the `SubImage`/`SubImageInner` - `SubImage::view()` (033),
`SubImage::sub_image()` mutable inherent method (034), and now `GenericImage::sub_image()` trait
default method (this candidate) all funnel into the same unguarded `Pixels::next()`.

## Introduced by

d0b2b7ba "Implement immutable and mutable pixel iterators. Implement an image trait to allow generic access to images. Provide ImageBuf for directly manipulating images. Provide DynamicImage for images whose pixel type is determined at runtime. Improve the Pixel trait Rearrange library(again)" (2014-07-20)

## The fix

Same fix as candidates 001/032/033/034 (re-verified here to also cover the trait-default
`sub_image()` entry point): an explicit early-out for `width == 0` at the top of `Pixels::next()`
(`src/images/generic_image.rs`), before the row-wrap logic that assumes a nonempty row.

```rust
fn next(&mut self) -> Option<(u32, u32, I::Pixel)> {
    if self.width == 0 {
        return None;
    }
    if self.x >= self.width {
        ...
```

Verified this single guard clause (identical to candidates 001/032/033/034's `patch.diff`, no
additional `GenericImage`-specific change needed) fixes both Case A (now yields 0 pixels, no
phantom data) and Case B (now yields 0 pixels, no panic) through this entry point too - see
`output.txt`.

- Project test suite (`cargo test --lib`, run against the main project's own working tree):
  **331 passed, 0 failed** before the patch -> **331 passed, 0 failed** after the patch (identical
  pass count, no new failures, no regressions).

`patch.diff` contains this fix as a diff against `base_commit`
(`44ce9226c541dd2b11897b9ef07156a28871cdb7`) - identical in content to candidates 001/032/033/034's
fix, since it is the same shared iterator being patched once for every entry point that reaches it.

## What's not done / open questions

- This is believed to be the last unexercised call site of this defect class flagged across
  candidates 001/032/033/034/035's investigation notes - no further `sub_image()`/`view()`
  construction path in `src/images/` is currently known to be unchecked. If a future refactor adds
  another way to construct a `SubImage`/`SubImageInner` (or another `GenericImageView` implementor
  with its own zero-width-capable view), it should be checked against the same `Pixels::next()`
  defect, since the actual bug lives in the shared iterator, not in any individual constructor -
  applying candidates 001/032/033/034's shared fix (already the same patch as this candidate)
  covers all of them at once regardless of which constructor is used.
- As with prior candidates in this family, only `Pixels`/immutable iteration was exercised. No
  mutable-iteration equivalent currently exists in the crate.
