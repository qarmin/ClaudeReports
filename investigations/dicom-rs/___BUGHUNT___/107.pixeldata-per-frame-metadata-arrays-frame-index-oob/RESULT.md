# pixeldata-per-frame-metadata-arrays-frame-index-oob

Follow-on sweep from `IDEAS.md`'s 2026-08-25 entry (recorded while working candidate 088):
"whether any other public entry point in `pixeldata`/`object`/`parser` has the same shape [a
length/count equality check that doesn't also bound the actual index/offset derived from a
separate caller argument] was not systematically re-swept". The `fragments[frame]` spot itself
(the exact thing 088 fixed) turned out to have an identical, already-known-and-confirmed twin in
`pixeldata/src/gdcm.rs` (candidate 060, "pixeldata-gdcm-fragments-index-oob") - re-discovered
independently this session, then recognized as a duplicate and not re-reported (see "What was
tried"). Continuing the sweep past that duplicate found a distinct, much more widely repeated
instance of the general "a `.len() > 1` check is used as a stand-in for `frame < len`, without
actually checking it" pattern: `pixeldata/src/lib.rs`'s `build_monochrome_image` (the `image`-
feature image-rendering path) and `convert_pixel_slice` (the plain numeric-array path used by
`to_vec_frame`/`to_ndarray_frame`, not gated behind any optional feature) both repeatedly use the
idiom `if X.len() > 1 { X[frame as usize] } else { X[0] }` for four different per-frame metadata
arrays - `RescaleIntercept`/`RescaleSlope`, VOI LUT Sequence, VOI LUT Function, and
`WindowCenter`/`WindowWidth` - **18 occurrences total** across both functions. None of these
arrays' lengths are cross-validated against the actual PixelData buffer's own real frame count
(only against each other, or in one case against the *declared* `NumberOfFrames` attribute, which
is itself never cross-validated against the buffer either) - so a `frame` argument that is in
range for the actual pixel data buffer (and therefore passes `frame_data()`'s own bounds check)
can still be out of range for one of these shorter per-frame arrays.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Grep sweep across `pixeldata/src`, `object/src`, `parser/src` for `.len() ==`/`.len() !=`/`.len() >` equality-or-threshold checks near a later `[` index expression using a separate variable, plus a direct grep for `fragments[`/`[frame`/`[idx`-shaped indexing | Static code reading, no build needed | Found the already-known `pixeldata/src/gdcm.rs:252` `fragments[frame]` spot (see below), `object/src/mem.rs`'s `apply` (already correctly bounds-checked via `.get_mut(...).ok_or_else(...)`, not a bug), and the 18-occurrence `X[frame as usize]` idiom in `pixeldata/src/lib.rs` described above |
| Cross-checked the `gdcm.rs:252` `fragments[frame]` spot against `___BUGHUNT___/SUMMARY.md` before building a repro for it | Read `SUMMARY.md`/`060.pixeldata-gdcm-fragments-index-oob/RESULT.md` | Exact duplicate: same file, same line, same root cause, same fix (`.get(frame).context(FrameOutOfRangeSnafu{..})`), already `Confirmed` as candidate 060 (found via git archaeology against `origin/fix/gdcm-fragments-oob`). Not re-reported as a new candidate; the half-built candidate folder and in-tree test used to independently verify it were deleted/reverted before continuing the sweep, per this skill's "don't re-investigate an already-covered spot" convention |
| Debug-assertions build, real public API, temporary in-tree `#[test]`s in `pixeldata/src/lib.rs` (reverted before this write-up): built two `FileDicomObject<InMemDicomObject>`s via `FileMetaTableBuilder`/`put_element` with `NumberOfFrames` declared as 2 but the raw `PixelData` buffer actually holding 4 frames' worth of bytes - one with a top-level VOI LUT Sequence (0028,3010) with 2 items, one with a `PerFrameFunctionalGroupsSequence`-derived `RescaleIntercept`/`RescaleSlope` with 2 entries (both matching the *declared* frame count, so the crate's own `voi_lut_sequence()`/`rescale()` accessor methods - which already separately guard against a VM/declared-`NumberOfFrames` mismatch - take their "matches declared frame count" branch and hand back the un-truncated 2-entry slice) | `cargo test -p dicom-pixeldata --features image --lib bughunt_ -- --nocapture`, 3 separate runs | Panics deterministically 3/3: `index out of bounds: the len is 2 but the index is 3` at `pixeldata/src/lib.rs:1083:42` (`to_dynamic_image(3)`, VOI LUT Sequence branch) and separately at `pixeldata/src/lib.rs:1647:33` (`to_vec_frame::<u16>(3)`, 16-bit rescale branch in `convert_pixel_slice` - reachable without the `image` feature at all) |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`, real public `dicom_pixeldata::PixelDecoder::decode_pixel_data()` + `DecodedPixelData::to_dynamic_image()`/`to_vec_frame()` API) against a fresh clone of pristine upstream `df2b9e42`, via `just test` | Release build (`cargo run --release`), 3 separate runs against the main tree plus one full `just test` run against a genuinely fresh clone | Reproduces both cases identically every time; see `output.txt` for the real captured `just test` output (before/after) |

## Status

**Confirmed** - deterministic, no layout/seed dependency; reproduces 3/3 in-tree debug-build test
runs, 3/3 standalone `repro/` runs against the main tree, and once more via a full `just test` run
against a genuinely fresh clone of pristine upstream `df2b9e42` - identical panic
messages/locations every time, for both of the two independently-checked occurrences of this
pattern (VOI LUT Sequence in the `image`-feature path, RescaleIntercept/RescaleSlope in the plain
numeric-array path).

**Fully reproducible**: **Yes**. `repro/` builds both DICOM objects entirely through this crate's
own public API (`FileMetaTableBuilder`, `InMemDicomObject`, `DataElement`, `DataSetSequence`) and
calls the real public `dicom_pixeldata::PixelDecoder::decode_pixel_data()` plus
`DecodedPixelData::to_dynamic_image()`/`to_vec_frame()` entry points - the same entry points any
real caller (a DICOM viewer rendering a multi-frame image, a tool exporting raw per-frame samples)
would use. This is a single external artifact (a DICOM object with an internal declared-vs-actual
frame-count mismatch) handed to the library's real public entry points; there is no second
participant to fake here (`CONFIRMATION.md` section 11's exception for a single artifact fed to a
parser's real public API applies the same way it did for candidate 088/096).

## Class

`crash` (an out-of-bounds slice-index panic reachable via the documented public
`DecodedPixelData::to_dynamic_image`/`to_vec_frame`/`to_ndarray_frame` methods on an ordinary
object obtained from `decode_pixel_data()` - no `unsafe`, no memory-safety implication beyond the
panic itself; the `to_vec_frame`/`to_ndarray_frame` half is reachable without any optional
feature enabled at all).

## Root cause

`build_monochrome_image` and `convert_pixel_slice` (`pixeldata/src/lib.rs`) repeatedly use the
idiom `if X.len() > 1 { X[frame as usize] } else { X[0] }` for four different per-frame metadata
arrays: `RescaleIntercept`/`RescaleSlope` (`self.rescale()`/`self.rescale` - one spot indexes the
raw field directly instead of the already-fetched, guarded `default` local, an apparent
copy-paste slip that happens to still need the same fix), the VOI LUT Sequence
(`self.voi_lut_sequence()`), VOI LUT Function (`self.voi_lut_function()`), and
`WindowCenter`/`WindowWidth` (`self.window()`) - **18 occurrences total**. The `.len() > 1` check
only proves the array has *more than one* entry, not that it has *at least `frame + 1`* entries;
the `else` branch's own `X[0]` fallback is the tell that the authors were already thinking about
"what if this array is shorter than expected", just not far enough to bound `frame` itself against
`X.len()`.

Crucially, each of these arrays' lengths comes from an attribute read entirely independently of
the raw `PixelData` buffer's own actual byte length: `NumberOfFrames` is a separate declared
attribute never cross-validated against the buffer size at all (as already established by
candidates 037/038/096 in this project's own `___BUGHUNT___/`); a top-level VOI LUT Sequence's
item count is read directly via `e.items()`, with no relationship to `NumberOfFrames` whatsoever;
and per-frame `RescaleIntercept`/`RescaleSlope`/window values sourced via
`PerFrameFunctionalGroupsSequence` get their length from that sequence's own item count, again
independent of the buffer. Two of the accessor *methods* used here
(`DecodedPixelData::rescale()`/`voi_lut_sequence()`) do already separately guard against their
own array's length not matching the *declared* `NumberOfFrames` (falling back to the first entry
with a `tracing::warn!` if it doesn't) - but that guard only protects against one specific kind of
mismatch (VM vs. declared `NumberOfFrames`), not the one this bug exploits (declared
`NumberOfFrames`/array length matching each other while the *actual* pixel data buffer holds more
frames than either declares) - `frame_data()`'s own bounds check is the only thing standing
between an out-of-range `frame` and a panic, and it only checks against the buffer's real byte
length, never against any of these metadata arrays.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Replaced each of the 18 `X[frame as usize]` occurrences (which the `if X.len() > 1` guard already
establishes is the *only* code that can execute when `X.len()` might be less than `frame + 1`)
with a bounds-checked lookup that falls back to the first entry on an out-of-range index, matching
the same "gracefully degrade to the first value" philosophy the crate's own `rescale()`/
`voi_lut_sequence()` accessors already use for their own (different) mismatch case, rather than
introducing a new error variant/behavior:

```rust
// Copy types (Rescale, VoiLutFunction, WindowLevel):
default.get(frame as usize).copied().unwrap_or(default[0])
// Reference types (VoiLut, not Copy):
voi_lut_sequence.get(frame as usize).unwrap_or(&voi_lut_sequence[0])
```

See `patch.diff` for the exact diff (scoped to `pixeldata/src/lib.rs` only, all 18 occurrences,
`cargo fmt`-clean).

Verified against more than the one triggering input per `CONFIRMATION.md` section 7: the two
`repro/` cases exercise two structurally different occurrences of the pattern (VOI LUT Sequence
via `to_dynamic_image`, RescaleIntercept/RescaleSlope via `to_vec_frame`, hitting different
functions/bit-depth branches/lines - `1083` and `1647`), both confirmed panicking before the fix
and both confirmed non-panicking after, via the same repro binary.

- Project test suite (`cargo test -p dicom-pixeldata --features image --lib`): **58 passed; 0
  failed; 2 ignored** both before and after the fix (identical counts, confirmed against the main
  tree with the fix applied and again after reverting it) - zero regressions. Also spot-checked
  `cargo test -p dicom-pixeldata --lib` (default features, no `image`): **39 passed; 0 failed; 2
  ignored**, unaffected either way. `cargo clippy -p dicom-pixeldata --features image --lib -- -D
  warnings` and `cargo fmt -p dicom-pixeldata -- --check` both pass clean on the patched file (the
  unrelated pre-existing `gdcm.rs` clippy warnings surfaced only when the `gdcm` feature is also
  enabled are untouched by this patch).

## What's not done / open questions

- The other 14 of the 18 occurrences (VOI LUT Function/window in both bit-depth branches of both
  functions, and the 8-bit rescale branches) were not each independently exercised with their own
  dedicated `repro/` case - they share the exact same textual idiom and the exact same fix, fixed
  by the same single patch, but only 2 of the 18 spots were directly proven to panic pre-fix and
  stop panicking post-fix. A future session wanting per-spot proof for the remaining 14 would need
  one more constructed object per spot (mismatched `WindowCenter`/`WindowWidth` via
  `PerFrameFunctionalGroupsSequence`, mismatched `VOILUTFunction`, and the 8-bit variants of the
  already-checked two).
- The `gdcm.rs:252` `fragments[frame]` spot (same general bug class, already `Confirmed` as
  candidate 060) is not re-covered by this patch - it lives in a different file/feature gate and
  already has its own separate fix recorded in candidate 060.
- Candidate 088's own `fragments[frame as usize]` spot in `pixeldata/src/lib.rs` (a *different*
  variable/branch - `DicomValue::PixelSequence`'s fragment lookup, not any of the four per-frame
  metadata arrays this candidate covers) is likewise not touched by this patch; it has its own
  separate fix recorded in candidate 088.
- Whether the equivalent per-frame metadata arrays in `pixeldata/src/gdcm.rs` (the `gdcm`-feature
  decode path, which has its own separate `rescale_intercept`/`voi_lut_sequence`/etc. reads via
  `pixeldata/src/attribute.rs`, feeding into `gdcm_rs` FFI calls rather than this file's own
  `Lut`/`WindowLevelTransform` machinery) have an analogous frame-index bug was not checked this
  session - `gdcm.rs`'s `decode_pixel_data`/`decode_pixel_data_frame` don't do their own per-frame
  LUT/window indexing in the same shape (they hand the whole arrays to GDCM's own C++ code via
  FFI), so this specific idiom likely doesn't recur there, but this wasn't independently verified.
- `object/src/mem.rs`'s `apply`/`apply_leaf` (the only other `.len() ==`-shaped check found in the
  `object`/`parser` crates during this sweep) was re-confirmed safe: it already uses
  `items.get_mut(*item as usize).ok_or_else(...)` rather than direct indexing, so no bug there.
  The rest of `parser/src` was swept via grep for the same shape but nothing else matching the
  "count check without a corresponding index bound" pattern was found this session beyond what's
  listed above; a longer/fuzzing-based sweep of `parser/src/dataset/read.rs` and
  `parser/src/stateful/decode.rs` specifically was not attempted (out of scope for this session's
  manual/boundary-value-focused approach).
