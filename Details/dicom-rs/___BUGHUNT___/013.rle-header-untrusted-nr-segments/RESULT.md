# RLE Lossless decoder panics on a malicious/malformed segment header

Found during a discovery round (no seeded lead): read `transfer-syntax-registry/src/adapters/
rle_lossless.rs` (RLE Lossless is a real, commonly-used DICOM pixel data compression transfer
syntax) against `HEURISTICS.md`'s "arithmetic/indexing on untrusted sizes" and "untrusted-length
preallocation" patterns - this file has no `#[cfg(test)] mod tests` at all (confirmed by grep, the
only major codec adapter in this crate without one), making it comparatively under-exercised
relative to the rest of the codebase.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `read_rle_header` (the private helper both `decode()` and `decode_frame()` call to parse an RLE segment's offset table) and its two call sites. Found: (1) `LittleEndian::read_u32(&fragment[0..4])` with no length check first; (2) `nr_segments` (an attacker-controlled `u32` read directly from the fragment, up to ~4.29 billion) drives both `vec![0; nr_segments as usize]` (the exact untrusted-length-preallocation pattern already confirmed and fixed elsewhere this session, candidate 004) and a header-size slice `&fragment[4..4*(nr_segments+1)]` with no bounds check against `fragment.len()`; (3) the per-segment slice `&fragment[offsets[ii]..offsets[ii+1]]` trusts each individually-attacker-controlled offset value with no validation; (4) `offsets[ii]`/`offsets[ii+1]` indexing itself assumes `nr_segments` is large enough for the image's `samples_per_pixel * bytes_per_sample`, with no check. |
| Standalone repro (release build, `dicom-encoding`+`dicom-transfer-syntax-registry`'s public API: a minimal `PixelDataObject` implementation + `TransferSyntax::pixel_data_reader()`) | `___BUGHUNT___/013.../repro`. A 16-byte fragment whose first 4 bytes declare `NumberOfSegments = 0xFFFFFFF0` (~4.29 billion), the rest zero-filled - nowhere near enough real data to back that many offset table entries. Obtained the RLE Lossless transfer syntax's registered pixel data reader from `TransferSyntaxRegistry` (the actual, real dispatch path `pixeldata`'s own decode pipeline uses) and called `.decode()`. | Reproduced deterministically: `range end index 17179869124 out of range for slice of length 16` panicking inside `read_rle_header`, from a single 16-byte, entirely attacker-controlled fragment. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic, no randomness in this code path. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`crash` (an unguarded slice-range panic, uncatchable by normal `Result` handling, reachable from a
16-byte crafted RLE fragment through the crate's real, ordinary decode dispatch path - not a
contrived internal-only call).

## Root cause

`read_rle_header` reads `NumberOfSegments` directly from untrusted fragment bytes with no upper
bound (PS3.5 Annex G.5 defines a maximum of 15 segments; the code enforced no maximum at all) and
uses it immediately, both to size a `Vec` allocation (`vec![0; nr_segments as usize]` - the same
infallible-allocation-from-untrusted-length anti-pattern as candidate 004, here independently
present in a different crate) and to compute a header-size slice range
(`&fragment[4..4*(nr_segments+1)]`) with no check that the fragment actually contains that many
bytes. A `nr_segments` value in the billions produces a slice end index far beyond the fragment's
real length, panicking immediately - well before the allocation-size concern even has a chance to
manifest as an OOM. Two further, closely related gaps in the same function pair (found while
designing the fix, not independently reproduced with their own separate minimal inputs) were fixed
alongside it: individual segment offset values (`offsets[ii]`/`offsets[ii+1]`) are themselves
attacker-controlled and were never validated against each other or against `fragment.len()` before
slicing; and `offsets.len()` was never checked against the number of segments the image's own
`BitsAllocated`/`SamplesPerPixel` actually requires before indexing into it, which could panic via
ordinary out-of-bounds `Vec` indexing (not just a slice-range panic) for a fragment with a
smaller-but-still-attacker-chosen segment count.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Changed `read_rle_header` to return `DecodeResult<Vec<u32>>` instead of an unconditional `Vec<u32>`,
validating (in order, each with a graceful `whatever!`-constructed error instead of a panic):
fragment has at least 4 bytes for the segment count itself; `nr_segments` is between 1 and 15
inclusive (PS3.5 Annex G.5's own defined bound); the fragment actually contains enough bytes for
the declared header size before slicing it. Both call sites (`decode()` and `decode_frame()` -
near-duplicated logic, matching this session's now-established pattern of parallel implementations
needing the same fix in more than one place, see candidates 006/007) now propagate this with `?`,
and additionally validate, right before each per-segment slice: that the segment index actually
exists in the offset table (protecting the `Vec` indexing), and that the resulting offset pair is
internally consistent and within the fragment's real bounds (protecting the slice itself). See
`patch.diff` (`transfer-syntax-registry/src/adapters/rle_lossless.rs`, one file, both call sites).

Verified: `repro/` re-run against the patched tree no longer panics - `decode()` now returns a
clear `Result::Err` (`"Invalid number of RLE segments: 4294967280"`) instead of a slice-index
panic.

- Project test suite: `cargo test -p dicom-transfer-syntax-registry -p dicom-pixeldata --lib
  --features dicom-transfer-syntax-registry/rle` - 39 + 3 (crate-local) passed / 0 failed / 2
  ignored, identical both before and after the patch - this includes `dicom-pixeldata`'s own
  integration tests against real RLE-compressed DICOM files (`pydicom/SC_rgb_rle.dcm`,
  `SC_rgb_rle_2frame.dcm`, `SC_rgb_rle_16bit.dcm`, `SC_rgb_rle_16bit_2frame.dcm` via
  `dicom-test-files`), confirming the new validation doesn't reject legitimate real-world RLE data.
  `cargo test --workspace --lib` - 351 passed / 0 failed / 2 ignored, identical both before and
  after (no regressions, no newly-passing tests - no existing test constructed a malformed RLE
  header).

## What's not done / open questions

- A related, **not independently confirmed with its own repro** panic risk was noticed while
  reading the same functions: `decoded_segment[decoded_index]` (later in both `decode()` and
  `decode_frame()`, after the segment slice this candidate fixed) indexes into a buffer whose
  actual length depends on how many bytes the inner `PackBitsReader` genuinely decoded from a
  segment's *compressed content* - if that content is truncated or malformed in a way that
  produces fewer than the expected `rows * cols` decompressed bytes (a different attack surface
  than this candidate's fix, which only addressed the segment *offset header*, not the
  *compressed payload* within an otherwise-valid segment), this indexing could panic too. Left
  unfixed and unconfirmed this session - worth a dedicated follow-up candidate with its own
  purpose-built malformed-PackBits-payload input.
- Not fuzzed - found by direct code reading against a known anti-pattern (candidate 004's
  untrusted-length-preallocation class, re-recognized in a different crate), then confirmed by
  hand-constructing a minimal malicious fragment. `pixeldata`'s own fuzz targets
  (`decode_image_file`/`decode_simple_image`) plausibly already exercise this code path if RLE
  Lossless is in their transfer syntax mix - not independently re-run against this fix.
