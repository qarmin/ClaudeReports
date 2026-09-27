# pixeldata-gdcm-fragments-index-oob

`pixeldata/src/gdcm.rs`'s `decode_pixel_data_frame` (the `PixelDecoder` impl used whenever the
`gdcm` feature is enabled, replacing the crate's native/pure-Rust decode path entirely) indexes
the pixel data's fragment list directly with the caller-supplied frame number, with no bounds
check:

```rust
let data = if number_of_frames == 1 && fragments.len() > 1 {
    fragments.iter().flat_map(|frame| frame.to_vec()).collect()
} else {
    fragments[frame].to_vec()
};
```

The `number_of_frames == 1 && fragments.len() > 1` branch handles one specific mismatch (a
single logical frame split across multiple fragments), but the `else` branch performs no
validation at all between `frame` and `fragments.len()` before indexing.

This is a distinct bug from already-investigated candidate 039
(`pixeldata-encapsulated-fragments-index-dead-code`, which examined an unrelated, currently
unreachable branch in `pixeldata/src/lib.rs`'s non-GDCM default decode path) - this one lives in
a different file (`gdcm.rs`, not `lib.rs`), a different feature-gated code path (`gdcm` feature,
not the default/native path), and is directly reachable, not dead code.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-059): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/gdcm-fragments-oob` (commit `0af753ca`) fixing this exact `fragments[frame]` line | Confirmed unmerged |
| Cross-checked against already-completed candidate 039 (same theme: unchecked fragment index in pixel data decode) | Read `039`'s `RESULT.md` and diffed the two branches' target files | Confirmed genuinely distinct: different file (`gdcm.rs` vs `lib.rs`), different feature gate (`gdcm` vs default), and this one is *not* dead code (039's was) |
| `cargo build -p dicom-pixeldata --features gdcm` | Confirms the `gdcm` feature (native GDCM bindings via `gdcm-rs`) builds in this sandbox | Builds cleanly, no native GDCM library missing |
| Temporary in-tree `#[test]` in `pixeldata/src/gdcm.rs` (reverted before write-up): open `pydicom/JPEG2000.dcm` (a real single-frame, single-fragment test file from `dicom-test-files`) and call `.decode_pixel_data_frame(1)` | `cargo test -p dicom-pixeldata --features gdcm gdcm_frame_oob_repro` | Panics: `index out of bounds: the len is 1 but the index is 1` at `pixeldata/src/gdcm.rs:252:21` |
| Standalone `repro/` (path-dependency on a pristine clone of `df2b9e42`, via `just test`) | `cargo run` | Reproduces the same panic deterministically; see `backtrace.txt` |

## Status

**Confirmed** - deterministic, reproduces every run, no fixed seed needed. Reproduced both via a
temporary in-tree test against the live tree and independently via `repro/` against a pristine
clone of the exact commit this session audits (`df2b9e42`).

## Class

`crash` (panic / index-out-of-bounds, reachable through this crate's own public
`PixelDecoder::decode_pixel_data_frame` API with an ordinary out-of-range frame request - no
malformed/adversarial DICOM content needed, just a frame index that exceeds the number of
fragments actually present).

## Root cause

The `else` branch's `fragments[frame]` assumes every requested frame maps to exactly one
fragment at that index, which is true for the common case but not validated - unlike this same
crate's non-GDCM native decode path (`pixeldata/src/lib.rs`), which already uses a
bounds-checked `.get(...)`-based accessor (`frame_pixel_data`) for the analogous operation. The
`gdcm.rs` module, added/maintained somewhat independently as an alternative optional decode
backend, didn't inherit that same guard.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Replace the direct index with `.get(frame)` and propagate `None` as the crate's own existing
`FrameOutOfRangeSnafu` error (already defined in `pixeldata/src/lib.rs` and already used
elsewhere in this same file for an analogous bounds condition), matching this project's own
established convention:

```rust
let data = if number_of_frames == 1 && fragments.len() > 1 {
    fragments.iter().flat_map(|frame| frame.to_vec()).collect()
} else {
    fragments
        .get(frame)
        .context(FrameOutOfRangeSnafu {
            frame_number: frame as u32,
        })?
        .to_vec()
};
```

See `patch.diff` for the exact diff (scoped to `pixeldata/src/gdcm.rs` only).

- `cargo build -p dicom-pixeldata --features gdcm`: builds cleanly with the fix applied.
- `cargo test -p dicom-pixeldata --features gdcm --lib` (against the patched pristine clone):
  **36 passed / 0 failed / 3 ignored** - unchanged from before the fix (minus the temporary
  repro test itself, removed before finalizing), confirming zero regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): panics before the patch, returns a clean `Err(FrameOutOfRange { .. })`
  after.
