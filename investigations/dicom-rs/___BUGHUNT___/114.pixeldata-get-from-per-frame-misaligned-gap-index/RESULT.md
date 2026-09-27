# pixeldata-get-from-per-frame-misaligned-gap-index

Follow-up to candidate 113 (backlog entry dated 2026-08-27, `IDEAS.md`): a survey of every
`DecodedPixelData` construction site in `pixeldata/src/lib.rs`/`gdcm.rs` for a caller that can
hand `DecodedPixelData::voi_lut_sequence()`'s (and its siblings') length-mismatch fallback
(`Ok(Some(&inner[0..1]))` with just a `tracing::warn!`, no error, whenever
`enforce_frame_fg_vm_match` is `false`) a genuinely mismatched-length per-frame metadata array,
at a call site other than the already-fixed-by-candidate-113 `decode_pixel_data_frame`. Found one:
the default (non-`gdcm`) whole-object `decode_pixel_data()` (not the per-frame variant), fed by
`pixeldata/src/attribute.rs`'s `get_from_per_frame` helper, which silently *skips* (via
`.filter_map`) any `PerFrameFunctionalGroupsSequence` item that doesn't carry the requested
attribute, rather than preserving that item's position - so the resulting `Vec` can end up shorter
than (and misaligned against) the real, declared `NumberOfFrames`, even while `decode_pixel_data()`
itself always sets `number_of_frames` to the real, correct value (unlike candidate 112/113's
hardcoded-`1` mechanism).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: traced all 7 `DecodedPixelData { ... }`/`Ok(DecodedPixelData { ... })` construction sites (`gdcm.rs:165`, `gdcm.rs:344`, `lib.rs:1958` `to_owned` (clones an already-built value, not a new construction), `lib.rs:2335`/`2387` (`decode_pixel_data`, encapsulated/native branches), `lib.rs:2486`/`2550` (`decode_pixel_data_frame`, already fixed by candidate 113 for `voi_lut_sequence` specifically)) against how each populates `voi_lut_sequence` vs. `number_of_frames` | Static reading only | `decode_pixel_data()` (whole-object, `lib.rs:2279`) passes `voi_lut_sequence` straight through from `ImagingProperties::from_obj`, with `number_of_frames` set to the *real* declared frame count (not hardcoded) - so at first glance this looked safe (unlike 112/113's mechanism), since a fully-populated `PerFrameFunctionalGroupsSequence` produces exactly `number_of_frames` entries. Then read `attribute.rs`'s `get_from_per_frame`: it uses `.iter().filter_map(...)` over the per-frame items, silently *dropping* any item that doesn't carry the target attribute (missing `FrameVOILUTSequence`, or present but missing the nested `VOILUTSequence`) instead of preserving its position - so a `PerFrameFunctionalGroupsSequence` where not every item carries the attribute produces a *shorter, misaligned* `Vec` even though `number_of_frames` is correct |
| Manual boundary-value construction: a real, public-API-built `FileDicomObject` with `NumberOfFrames: 3`, 3 real native uncompressed frames (pixel bytes `[10,10,10,10]`/`[20,20,20,20]`/`[30,30,30,30]`), and a real `PerFrameFunctionalGroupsSequence` with 3 items - item 0 and item 2 each carry a real, distinct, correctly-nested `FrameVOILUTSequence`/`VOILUTSequence` single-entry LUT (item 0 maps every input to 0, item 2 maps every input to 255), item 1 carries **no** `FrameVOILUTSequence` at all (a real-world-plausible gap: a non-fully-compliant per-frame functional groups sequence, not every macro necessarily present in every item), decoded via the default (non-`gdcm`) whole-object `decode_pixel_data()` | Debug build in-tree first (`cargo test -p dicom-pixeldata --features image --lib`, a temporary `#[test]`, reverted before write-up), then release build via a standalone `repro/` against a fresh clone of pristine upstream `df2b9e42` | Reproduced deterministically: `decode_pixel_data().to_dynamic_image(2)` (frame 2, which has its own real, present per-frame VOI LUT mapping to 255) returns pixel value `0` (frame 0's LUT) instead of `255` - 3/3 in-tree debug runs, 3/3 standalone release runs against the fresh clone (`just clone && just reproduce-before`, run 3 times) |

## Status

**Confirmed**, reproduces 3/3 in-tree debug-build runs and 3/3 standalone `repro/` runs against a
fresh clone of pristine upstream `df2b9e42` (deterministic logic bug, not layout/seed-dependent -
every run gives the identical wrong pixel value `0` instead of the correct `255`).

**Fully reproducible**: **Yes**. This is a single-participant finding (one caller decoding one
real DICOM object through the crate's own public `PixelDecoder`/`DecodedPixelData` API -
`obj.decode_pixel_data().unwrap()` then `.to_dynamic_image(frame)`, the same entry point any real
caller - a DICOM viewer, a PACS client - would use), not an interaction between two independent
participants - see `CONFIRMATION.md` section 11's carve-out for a single artifact fed to a
library's real public entry point; there is no second participant to fake here.

## Class

logic

## Root cause

`pixeldata/src/attribute.rs`'s `get_from_per_frame` (used by `rescale_intercept`,
`rescale_slope`, `voi_lut_function`, `window_center`, `window_width`, and `voi_lut_sequence` to
read a per-frame value out of `PerFrameFunctionalGroupsSequence`) iterates the sequence's items
with `.filter_map(move |item| item.get(selector[0])?.items()?.first()?.get(selector[1]))`: any
item that doesn't carry the requested attribute (missing macro sequence, or present but empty) is
simply **skipped**, not kept as a placeholder. The resulting `Vec`'s index `i` therefore does
*not* necessarily correspond to frame `i` once any item in the middle of the sequence lacks the
attribute - and, separately, the `Vec`'s length can end up strictly shorter than the real,
correctly-declared `NumberOfFrames`.

`decode_pixel_data()` (the default, non-`gdcm`, whole-object decode in `pixeldata/src/lib.rs`)
sets `number_of_frames` to the real frame count and hands `voi_lut_sequence` straight through
unmodified from `ImagingProperties::from_obj` - unlike candidate 112/113's hardcoded-`number_of_frames:
1` mechanism, this path has no narrowing step to skip in the first place, and superficially looks
correct because the array's length usually equals `number_of_frames` when every per-frame item is
fully populated. But when even one item in the middle is missing the attribute, the array becomes
both shorter than `number_of_frames` *and* misaligned (each surviving entry shifts up to fill the
gap). `DecodedPixelData::voi_lut_sequence()`'s length-mismatch guard (`len == number_of_frames`)
then correctly detects the shortened length as a mismatch, but because `decode_pixel_data()`
always sets `enforce_frame_fg_vm_match: false`, the mismatch doesn't error - it silently returns
`Ok(Some(&inner[0..1]))`, i.e. element 0 of the (already-misaligned) array, for every frame,
regardless of which frame's own real per-frame VOI LUT was actually present in the object.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed `get_from_per_frame`'s contract from "skip items missing the attribute" to "the per-frame
source is only usable if every item carries the attribute" - it now returns `None` (falling
through to `get_from_shared`, then to the caller's own top-level-attribute-absent default) as soon
as any single item is missing it, instead of silently returning a shorter, index-misaligned `Vec`:

```rust
fn get_from_per_frame<D: DataDictionary + Clone>(
    obj: &FileDicomObject<InMemDicomObject<D>>,
    selector: [Tag; 2],
) -> Option<Vec<&InMemElement<D>>> {
    let items = obj.get(tags::PER_FRAME_FUNCTIONAL_GROUPS_SEQUENCE)?.items()?;
    if items.is_empty() {
        return None;
    }
    items
        .iter()
        .map(|item| item.get(selector[0])?.items()?.first()?.get(selector[1]))
        .collect()
}
```

(`Vec<_>::collect()` over an iterator of `Option<_>` short-circuits to `None` on the first `None`,
via `Option<Vec<_>>: FromIterator<Option<_>>`.) One call site (`voi_lut_function`) needed a small
adjustment since the return type changed from `impl Iterator` to `Vec` (removing a now-redundant
`.map(|inner| inner.collect())`); the other five call sites (`rescale_intercept`, `rescale_slope`,
`window_center`, `window_width`, `voi_lut_sequence`) already consume the result via
`.into_iter()...collect()`, which works identically over a `Vec`. See `patch.diff`.

This is a conservative choice: a `PerFrameFunctionalGroupsSequence` where an attribute is present
in some items but not others is already non-compliant with the DICOM standard (per-frame macros
are expected to be either present in every item or shared); treating that whole attribute as
unavailable (falling through to whatever the caller does when the attribute is fully absent, e.g.
`decode_pixel_data()`'s own rescale/window/normalize fallbacks) is safer than silently returning
one unrelated frame's data for every other frame.

Verified: re-ran the same repro against the patched tree (in-tree, 3/3 runs, and via `repro/`'s
`just apply && just reproduce-after` against the applied clone, 1/1) -
`decode_pixel_data().to_dynamic_image(2)` no longer returns `0`; `voi_lut_sequence()` now reports
`None` (the whole per-frame VOI LUT source is treated as unavailable, since item 1 lacks it) and
the pixel value falls back to the normalize-based Modality LUT path, landing on `255` for this
particular single-valued frame - the wrong, cross-frame-contaminated value (`0`, frame 0's LUT) is
gone either way.

- Project test suite: `cargo test -p dicom-pixeldata --features image --lib` (with the same
  temporary reproduction test present, then reverted before write-up): **58 passed / 1 failed / 2
  ignored** before the fix (the added reproduction test fails, everything else passes), **59
  passed / 0 failed / 2 ignored** after the fix (identical pre-existing 58 plus the now-passing
  new test) - zero regressions among the pre-existing 58. `cargo test --workspace --lib` after the
  fix (fix + test both present): **352 passed / 0 failed / 2 ignored** total across the workspace
  (matches candidate 113's recorded baseline of 351 passed/0 failed/2 ignored, plus the one new
  test added this session).

## What's not done / open questions

- `get_from_per_frame` is also used by `rescale_intercept`/`rescale_slope`/`window_center`/
  `window_width` (not just `voi_lut_sequence`/`voi_lut_function`); this fix changes all of their
  behavior identically (a partially-populated per-frame source becomes fully unavailable rather
  than silently misaligned), but only `voi_lut_sequence` was independently exercised with its own
  dedicated repro this session - the other four share the exact same helper and the exact same
  fix, but weren't each separately proven wrong-before/correct-after.
- `get_from_shared` (the `SharedFunctionalGroupsSequence` sibling of `get_from_per_frame`) has no
  equivalent bug: it only ever reads the *first* item of the shared sequence (by DICOM design,
  shared groups apply to all frames uniformly), so there is no per-frame index to misalign in the
  first place - not a live suspect, not changed.
- `gdcm.rs`'s two `DecodedPixelData` construction sites (`decode_pixel_data`/
  `decode_pixel_data_frame`) call their own separate `voi_lut_sequence`/`rescale_intercept`/etc.
  functions in `attribute.rs`, which share the exact same (now-fixed) `get_from_per_frame` helper
  - so this fix also covers the `gdcm`-feature path for the same misalignment mechanism, though
  this wasn't independently exercised with a `gdcm`-feature repro this session (the `gdcm` feature
  requires the vendored GDCM C++ library and wasn't built for this candidate).
- The general "silent fallback to element 0 with just a `tracing::warn!`" shape in
  `DecodedPixelData::voi_lut_sequence()`/`rescale()`/`window()`/`voi_lut_function()` themselves
  (the accessor methods, not `get_from_per_frame`) is unchanged by this fix - it's still reachable
  in principle by any future caller that constructs a `DecodedPixelData` with a genuinely
  mismatched-length array and `enforce_frame_fg_vm_match: false`; this fix closes the one concrete
  mechanism found this session (`get_from_per_frame`'s silent item-skipping) that could produce
  such a mismatch from real DICOM input through the crate's existing public construction sites,
  but does not change the accessors' own fallback behavior, which remains shared by other callers
  and out of scope here (same open point already recorded by candidate 113).
