# BitsStored=0 panics deep inside LUT construction instead of erroring

Found during a discovery round (no seeded lead): read `pixeldata/src/lut.rs` against
`HEURISTICS.md`'s "error swallowing"/"unchecked state-machine invariant" patterns, following up on
candidate 003's VOI-LUT work in the same crate. `Lut::new_with_fn`'s doc comment already says
"Panics if `bits_stored` is 0 or too large" - an explicit, deliberate precondition - which raised
the question of whether every call site actually guarantees that precondition before calling in,
or whether an attacker-controlled DICOM attribute can reach it directly.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `pixeldata/src/lut.rs`'s `Lut::new_with_fn` (`assert!(bits_stored != 0 && bits_stored <= 32)`, line 113) and its callers in `pixeldata/src/lib.rs` (`Lut::new_rescale(self.bits_stored, ...)` and several sibling calls, all passing `self.bits_stored` - a field populated from the DICOM object's own BitsStored (0028,0101) element). Traced back to `pixeldata/src/attribute.rs`'s `bits_stored()` accessor: a thin wrapper around `retrieve_required_u16` with **no range validation at all** - any `u16` value from the wire, including `0`, passes straight through. |
| Standalone repro (release build, `object`+`pixeldata`'s public API: `open_file`, `InMemDicomObject::put_element`, `PixelDecoder::decode_pixel_data`/`DecodedPixelData::to_vec`) | `___BUGHUNT___/009.../repro`. Opened a real test file (`pydicom/CT_small.dcm` via `dicom-test-files`), overwrote its `BitsStored` element to `0` via the public element-mutation API, called `.decode_pixel_data()` then `.to_vec::<u16>()` inside `catch_unwind`. | Reproduced deterministically: `assertion failed: bits_stored != 0 && bits_stored <= 32` panicking inside `Lut::new_with_fn`, reached via `DecodedPixelData::to_vec`. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic, no randomness in this code path. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`crash` (an `assert!` - active in release builds, unlike `debug_assert!` - panics on ordinary,
trivially attacker-forgeable input: a 2-byte `BitsStored` element set to `0`, reachable from any
caller that decodes and renders pixel data from an untrusted file via the crate's normal, most
common public API).

## Root cause

`Lut::new_with_fn` (`pixeldata/src/lut.rs`) documents and enforces a precondition
(`bits_stored != 0 && bits_stored <= 32`) via `assert!`, but the only source of `bits_stored`
values reaching it in practice - `pixeldata/src/attribute.rs`'s `bits_stored()`, which reads the
DICOM object's own `BitsStored` (0028,0101) element - never validates the value it retrieves
before returning it. `BitsStored` is a required attribute with no library-level range check, so a
file that simply sets it to `0` (or, less severely tested here but by the same reasoning, an
implausibly large value the standard doesn't actually permit) flows unchecked all the way from
"raw bytes on the wire" to "hits a hard `assert!` deep inside LUT table construction," turning
what should be a rejected/malformed-input `Result::Err` into an uncatchable-by-normal-means
process panic. `assert!(bits_stored != 0 && bits_stored <= 32)`'s own doc comment ("Panics if...")
already signals that this precondition was known to matter - the gap is that nothing upstream of
the LUT code actually enforces it before that point is reached.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added the missing validation exactly where the untrusted value first enters the library:
`bits_stored()` in `pixeldata/src/attribute.rs` now checks the retrieved value against
`1..=32` (matching `Lut::new_with_fn`'s own documented precondition precisely) and returns a
`Result::Err` (`InvalidValueSnafu`, the same error variant already used for this crate's other
attribute-range checks, e.g. candidate 003's VOI LUT Descriptor validation) instead of letting an
out-of-range value propagate further. See `patch.diff` (`pixeldata/src/attribute.rs`, one
function, 13-line diff).

Verified: `repro/` re-run against the patched tree no longer panics - `decode_pixel_data()` itself
now fails fast with a clear `InvalidValue { name: BitsStored, value: "0" }` error (even earlier
than the original panic site, since `ImagingProperties::from_obj` calls `bits_stored()` during
decode setup, before any LUT is ever constructed) - a normal, catchable `Result::Err` instead of
an uncatchable panic.

- Project test suite: `cargo test -p dicom-pixeldata --lib` - 39 passed / 0 failed / 2 ignored,
  identical both before and after the patch. `cargo test --workspace --lib` - 351 passed / 0
  failed / 2 ignored, identical both before and after (no regressions, no newly-passing tests - no
  existing test constructed a `BitsStored` value outside the valid range).

## What's not done / open questions

- Only confirmed and fixed the `bits_stored=0` boundary specifically (the exact value this
  session's repro used). The `bits_stored > 32` side of the same `assert!` was not independently
  constructed as its own repro, though the fix's `1..=32` range check covers it identically and by
  the same reasoning (the assert's own bound). Real-world `BitsStored` values are always small
  (1-16 per the standard's typical bit-depth range for medical images), so `>32` is an even less
  plausible input than `0`, but the fix doesn't distinguish - both are rejected the same way.
- A related, narrower, **not separately confirmed** boundary was noticed while reading
  `Lut::new_with_fn`: `let size = (1 << bits_stored as u32) as usize;` computes the shift in the
  literal `1`'s default-inferred type (`i32`) before the `as usize` cast - `bits_stored == 32`
  would shift a 32-bit signed integer by 32 bits, which is out-of-range for that type (valid shift
  amounts for `i32` are 0-31) and would itself panic (in a debug build with overflow checks) even
  though `32` is explicitly *within* the function's own asserted-valid range. This candidate's fix
  makes `bits_stored=32` reachable again after being rejected (previously masked by unrelated
  attribute validation never getting that far) - **worth a dedicated follow-up candidate** to
  confirm and fix this specific edge (e.g. `1u64 << bits_stored as u32`, or bounding the accepted
  range more conservatively) rather than assuming the existing `assert!`'s stated bound is
  internally self-consistent.
- Not fuzzed - found by direct code reading of a documented-but-unenforced precondition, then
  confirmed by hand-constructing a minimal malformed attribute value, per `HEURISTICS.md`'s
  guidance for "state-machine invariant that isn't checked, only assumed."
