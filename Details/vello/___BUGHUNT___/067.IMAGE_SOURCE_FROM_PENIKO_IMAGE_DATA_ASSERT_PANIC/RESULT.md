# IMAGE_SOURCE_FROM_PENIKO_IMAGE_DATA_ASSERT_PANIC

`vello_common::paint::ImageSource::from_peniko_image_data(image: &peniko::ImageData)` is a public
API meant to convert a `peniko::ImageData` (a plain data struct with `u32` `width`/`height` that
can come straight from a decoded image file - PNG/JPEG/an SVG-embedded raster) into vello's
internal `ImageSource`. It `assert!`s that `width`/`height` are both `<= u16::MAX`, then panics
(the doc comment already says "# Panics" for this case) instead of returning `Result`/`Option`.
This looked suspicious because dimensions decoded from an untrusted external image are exactly the
kind of value that should be validated at the boundary rather than trusted all the way into an
`assert!`, and because vello's own example image-loading code (`examples/scenes/src/images.rs`)
passes a real decoder's `width()`/`height()` straight into `ImageData` with no size check anywhere
in between.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction, real end-to-end path: a real PNG file (70000x1 RGBA, built with the `png` crate) decoded via the exact same `image::ImageReader::new(...).with_guessed_format()?.decode()?` pattern `examples/scenes/src/images.rs::decode_image` uses, producing a genuine `peniko::ImageData`, then passed to the real public `ImageSource::from_peniko_image_data` | debug and `--release` builds, standalone crate depending on `vello_common`'s real published API | Panics every run, both builds: `"The image is too big. Its width and height can be no larger than 65535 pixels."` at `sparse_strips/vello_common/src/paint.rs:167` |
| Repeat runs of the same repro | identical build/input, 4 additional runs (5/5 total) in `--release`, plus 1 in plain `cargo run` (debug) | Panicked identically every time - fully deterministic, no memory-layout/seed dependency (this is a direct integer comparison, not something needing miri/ASAN/fuzzing to surface) |
| Code reading: any in-repo caller that already validates dimensions before calling this function | `grep -rn from_peniko_image_data` across the whole repo | No caller anywhere in this repo besides its own unit tests - it is purely a leaf public API for an external embedder to call directly, exactly the shape `examples/scenes/src/images.rs`'s own `decode_image` would produce |

Only one approach class was needed (manual boundary-value construction against a real decoded
file) - this is a plain, deterministic assert/panic on an out-of-range value, not something that
needs miri/ASAN/a fuzz harness to surface.

## Status

**Confirmed**: reproduces 5/5 runs in `--release`, plus in a plain debug `cargo run` - both
against a pristine, freshly cloned checkout at the base commit (see `repro/`/`output.txt`).
Fully deterministic (a direct `u32 > u16::MAX` comparison, no seed/layout dependency).

**Fully reproducible**: **Yes**. The repro's only external artifact is a real, valid 70000x1 PNG
file built with the `png` crate - the same class of thing an attacker/user would hand a real
application (an oversized image file, or an SVG with an embedded raster of that size). It is
decoded with the real `image` crate's real public decode entry point
(`image::ImageReader::new(...).with_guessed_format()?.decode()?`), using the identical pattern
this very repo's own `examples/scenes/src/images.rs::decode_image` uses, and the resulting
`peniko::ImageData` (genuine decoded `width`/`height`, not hand-set) is passed straight into
`vello_common`'s real, fully public `ImageSource::from_peniko_image_data`. Per `CONFIRMATION.md`
section 11, this is the "malformed file handed to a parser's real public entry point" case, not
the "faked second participant" case - there is no separate peer/caller being impersonated here,
the file itself *is* the genuine external input, so a single-process repro is the correct shape
(no network/IPC needed).

## Class

crash (documented panic on a public API's own, otherwise-unvalidated `u32` input field)

## Root cause

`ImageSource::from_peniko_image_data` (`sparse_strips/vello_common/src/paint.rs:164-173`) does:

```rust
assert!(
    image.width <= u16::MAX as u32 && image.height <= u16::MAX as u32,
    "The image is too big. Its width and height can be no larger than {} pixels.",
    u16::MAX,
);
let width = image.width.try_into().unwrap();
let height = image.height.try_into().unwrap();
```

`vello_common`'s internal `Pixmap` (and everything downstream in the tiling/rasterization
pipeline) represents image dimensions as `u16`, but `peniko::ImageData::width`/`height` are `u32` -
the type that upstream image decoders (the `image` crate, a PNG/JPEG parser, an SVG's embedded
raster) naturally produce, with no inherent ceiling at `u16::MAX`. Converting between the two
representations is a real, expected occurrence (any image wider or taller than 65535px, however
unusual, decodes to a `u32` width/height with no error from the decoder itself - a 70000x1 image,
as built here, is only 355 bytes as a PNG file and decodes near-instantly). The function chose to
enforce vello's own internal `u16` limit via `assert!` (i.e. abort the whole process) rather than
by returning `Result`/`Option`, even though the doc comment already flags this exact case as a
"# Panics" contract - so it is a *documented* panic, not a hidden one, but the public API shape
still means any caller passing through externally-influenced dimensions without its own prior
bounds check inherits an abort rather than a recoverable error.

No caller inside this repository validates `width`/`height` before calling this function (it has
no in-repo callers at all besides its own unit tests) - `examples/scenes/src/images.rs`'s own
`decode_image` helper is the closest realistic analogue of a caller assembling a `peniko::ImageData`
from a real decoded file, and it performs no size check on `image.width()`/`image.height()` before
constructing `ImageData`, so an application built the same way (loading arbitrary user-supplied
image files, exactly the shape `vello`'s own example loader uses) would panic the whole process on
such a file if it later called `from_peniko_image_data` on the result.

## Introduced by

b3e173e8 "Implement `Image::from_peniko_image` (#996)" (2025-05-09) - this commit first added
`from_peniko_image_data` (originally on a slightly different receiver type, later renamed by
bf4962a0 "sparse_strips: Use peniko::Brush (#1253)" (2025-10-02) to its current signature, and by
3fabef93 "vello_common: Unify and improve constructor for pixmaps (#1834)" to its current
`Pixmap::from_parts`-based body) with the same `assert!`-and-abort behavior on oversized
dimensions present from the start; found via `git log -S "The image is too big"`.

## The fix

Changed `from_peniko_image_data`'s signature from `-> Self` to `-> Option<Self>`: the `assert!` was
replaced with an early `return None` for the same oversized-dimension condition, and the final
`Self::Pixmap(...)` construction is now wrapped in `Some(...)`. The doc comment's "# Panics"
section was replaced with a description of the `None` return case. This removes the only path by
which an out-of-range `width`/`height` reaches the process-aborting `assert!` (or the
`try_into().unwrap()` calls right after it, which would panic identically even without the
`assert!`), while keeping the in-range path's behavior completely unchanged.

This is a breaking API change (return type changes from `ImageSource` to `Option<ImageSource>`),
consistent with this crate's own changelog conventions, which already record breaking changes of
similar shape for other public APIs (see `sparse_strips/vello_common/CHANGELOG.md`'s "Breaking
change" entries). The three existing unit tests in `paint.rs` that call this function were updated
to `.unwrap()` the (guaranteed-`Some`, since their inputs are always in-range) result, and a new
test (`from_peniko_image_data_rejects_oversized_dimensions`) was added, checking both the
width-too-big and height-too-big cases independently (per `CONFIRMATION.md` section 7 - not just
the exact one input class the original repro used).

Verified by re-running the exact same standalone `repro/` against the patched tree with a small,
signature-aware `--features fixed` call-site switch (see `repro/src/main.rs`'s
`#[cfg(feature = "fixed")]` branch) - same decoded oversized PNG, same call, now returns `None`
instead of panicking (see `output.txt` for the full before/after transcript, captured from an
actual fresh clone + `git apply` + rerun, not reconstructed).

- Project test suite (`vello_common` + `vello_cpu` + `vello_hybrid`, `--all-features`): 372 passed,
  0 failed before -> 373 passed, 0 failed after (the +1 is the new
  `from_peniko_image_data_rejects_oversized_dimensions` test; no other test's outcome changed).
  Full workspace (`cargo build --workspace --all-features`) also builds cleanly against the patched
  tree with no other in-repo caller needing changes (there are none besides `paint.rs`'s own unit
  tests).

## What's not done / open questions

- `vello_common` is a published, semver-versioned crate - this fix is a breaking change to a
  public function's return type and would need a version bump + changelog entry (not added here,
  since this workspace's own rule is to always revert the main project's working tree - see
  `patch.diff` for what a real PR would need to additionally touch in
  `sparse_strips/vello_common/CHANGELOG.md`).
  - An additive, migration-friendly alternative (`try_from_peniko_image_data` returning
    `Option<Self>`, leaving `from_peniko_image_data` panicking as-is for existing callers relying
    on `Self`) was considered but not chosen: the panicking constructor's whole reason for existing
    is to be *the* conversion function for untrusted `peniko::ImageData`, so leaving it panicking
    while adding a second, safer sibling would just relocate the same footgun rather than remove
    it, and this crate's own changelog already documents breaking changes of this shape as an
    accepted way to fix similar issues.
- Any real out-of-repo embedder/frontend built on `vello_common` (an SVG renderer, an image-loading
  pipeline) that currently calls `from_peniko_image_data` directly and relies on it never returning
  (treating its `Self` return as infallible) would need its own call site updated for the new
  `Option<Self>` - this is exactly what a semver-major bump + changelog entry communicates.
- Not checked: whether `vello_svg` (a separate crate, not present in this repository) or any other
  out-of-tree consumer already validates dimensions before calling this function - `vello_svg`'s
  source isn't available in this checkout to inspect.
