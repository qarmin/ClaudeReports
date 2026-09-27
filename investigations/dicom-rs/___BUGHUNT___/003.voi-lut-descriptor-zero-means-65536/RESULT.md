# VOI LUT Descriptor value 0 (meaning 65536 entries) rejected, silently drops the whole sequence

Found during a discovery round (no seeded lead): read `pixeldata/src/attribute.rs` against
`HEURISTICS.md`'s "error swallowing that turns a real failure into silent wrong behavior" pattern,
focusing on `pixeldata` (untrusted image/LUT data, less fuzzed by the prior campaign than
`object`/`parser`). `parse_voi_lut_entry`'s handling of the VOI LUT Sequence's `LUT Descriptor`
(0028,3002) first value stood out: it rejects `0` outright, but PS3.3 C.7.6.3.1.5 documents `0` as
a deliberate special-case encoding for 65536 entries, not an invalid/empty length.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `pixeldata/src/attribute.rs`'s `parse_voi_lut_entry` (LUT Descriptor validation) and `voi_lut_sequence` (the function that calls it once per sequence item via `.map(|e| parse_voi_lut_entry(e).ok()).collect::<Option<Vec<VoiLut>>>()`). | Confirmed `ensure!(descriptor_elements[0] > 0, ...)` rejects the standard's `0` encoding, and that `.ok()` + `collect::<Option<Vec<_>>>()` means a SINGLE rejected item makes the ENTIRE sequence disappear (`None`), not just that one item - an error-swallowing amplifier on top of the underlying misinterpretation. Also checked the only other `LUT_DESCRIPTOR`-consuming call site in the crate (`grep`, one hit) - no Modality LUT / Presentation LUT equivalent exists in this codebase to share the same bug. |
| Standalone repro (release build, `object`'s and `pixeldata`'s public APIs: `open_file`, `InMemDicomObject::put_element`, `PixelDecoder::decode_pixel_data`/`voi_lut_sequence`) | `___BUGHUNT___/003.../repro`. Opened a real test file (`pydicom/CT_small.dcm` via the `dicom-test-files` crate, same dependency already used by the project's own pixeldata tests), injected a synthetic but standard-conformant VOI LUT Sequence item (`LUT Descriptor = [0, 0, 16]`, `LUT Data` = 65536 real `u16` entries matching the declared length), decoded pixel data, called `.voi_lut_sequence()`. | Reproduced deterministically: `voi_lut_sequence()` returned `Ok(None)` instead of `Ok(Some(&[one entry]))` - the conformant LUT was silently treated as if it didn't exist at all. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` (pre-fix) | 3/3 runs reproduced identically - fully deterministic (no randomness anywhere in this code path; only stopped at 3 runs rather than the usual 5 because construction takes noticeably longer per run, building a real 65536-entry LUT and decoding a real test image each time - still comfortably over the repeatability bar). |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`logic` (error swallowing: a real, standard-conformant input is silently treated as absent rather
than either being correctly parsed or surfacing a visible error).

## Root cause

`parse_voi_lut_entry` (`pixeldata/src/attribute.rs`) reads the VOI LUT Sequence's `LUT Descriptor`
element and uses its first value directly as the expected `LUT Data` entry count:
`descriptor_elements[0] as usize`, after an `ensure!(descriptor_elements[0] > 0, ...)` check that
rejects `0` as invalid. But PS3.3 C.7.6.3.1.5 explicitly documents `0` as meaning **65536**
entries - the actual count doesn't fit in the same 16-bit field used to encode the descriptor
value, so `0` is the standard's own escape hatch for the maximum size, not an indicator of an
empty/invalid LUT. Every real-world VOI LUT with exactly 65536 entries (the maximum a single VOI
LUT table can have) triggers this: `parse_voi_lut_entry` returns `Err`, and the caller,
`voi_lut_sequence()`, discards that per-item error with `.ok()` before folding all items through
`collect::<Option<Vec<VoiLut>>>()` - a `None` from any single item makes the whole `Option`
collection short-circuit to `None`, so the caller of `.voi_lut_sequence()` sees "no VOI LUT
sequence present" instead of either the correct 65536-entry LUT or a visible error. Anything
downstream defaults to a different rendering path (window/rescale-based) instead of the file's own
specified VOI LUT - a silent, wrong-image-rendering correctness bug for a real, valid file, not
just a rejected edge case.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed the LUT length validation in `parse_voi_lut_entry` (`pixeldata/src/attribute.rs`) to
accept `0` and interpret it as `65536`, while still rejecting genuinely negative values (which
have no valid interpretation under either reading). See `patch.diff` (one function, 7-line diff).
Verified: `repro/` re-run against the patched tree now reports the VOI LUT Sequence correctly
recognized with its 1 entry (rather than silently dropped), for the exact same input.

- Project test suite: `cargo test -p dicom-pixeldata --lib` - 39 passed / 0 failed / 2 ignored,
  identical both before and after the patch. `cargo test --workspace --lib` - 351 passed / 0
  failed / 2 ignored, identical both before and after (no regressions, no newly-passing tests - no
  existing test constructed a 65536-entry VOI LUT).

## What's not done / open questions

- Only the VOI LUT Sequence's `LUT Descriptor` handling was checked - this is the only
  `LUT_DESCRIPTOR`-consuming code path in the `pixeldata` crate (confirmed by grep), so there is no
  sibling Modality LUT / Presentation LUT parser sharing this exact bug in this codebase (unlike
  candidate 001, where the same root cause genuinely repeated in two functions).
- Did not investigate whether any *other* DICOM attribute in this codebase uses a similar
  "0 means max, not empty" encoding convention (a pattern that appears in a few other DICOM fields
  historically, e.g. some overlay/curve length fields) - not checked here, would need a dedicated
  pass through PS3.3 if pursued.
- Not fuzzed - found by direct code reading against a documented standard convention, then
  confirmed by hand-constructing a conformant input, per `HEURISTICS.md`'s guidance that a
  state-machine/format invariant violation is best confirmed by manual construction rather than a
  fuzz harness when the exact triggering condition is already known from the standard.
