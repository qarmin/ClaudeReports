# JPEG 2000 decoder panics when the DICOM object's declared dimensions don't match the codestream's

From `IDEAS.md`'s backlog, itself noticed while investigating candidate 015
(`015.deflate-bomb-unbounded-decompression`): checked the other pixel data adapters that delegate
to external image codec crates rather than hand-written glue, to see if any shared the same
"trusts external decoder output size implicitly" risk profile. `jpeg2k.rs` does, but via a
different specific mechanism than the deflate bomb: it pre-sizes its output buffer from the DICOM
object's own declared `Rows`/`Columns`, then writes the *actual* decoded JPEG 2000 image's samples
into that buffer with no check that the two dimension sources agree.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `Jpeg2000Adapter::decode_frame` (`transfer-syntax-registry/src/adapters/jpeg2k.rs`) - `dst` is sized to `samples_per_pixel * bytes_per_sample * cols * rows` (the DICOM object's own declared dimensions) *before* the JPEG 2000 codestream is even decoded, then `Image::from_bytes` decodes the *actual* embedded codestream (which has its own, entirely independent width/height/component-count baked into the JP2/J2K format itself) and the result is written into `dst` at offsets computed from the decoded image's own sample count - with no check that the decoded sample count actually fits within the pre-sized buffer. Also noticed a related off-by-one: the "too many components" guard used `component_i > samples_per_pixel` instead of `>=`, an inclusive/exclusive boundary slip (fixed alongside, though not independently confirmed as reachable on its own - `>=` is simply the intended, spec-correct check for a valid `0..samples_per_pixel` component index range). |
| Standalone repro (release build, `dicom-object`+`dicom-pixeldata`'s public API: `dicom_object::from_reader`, `InMemDicomObject::put_element`, `PixelDecoder::decode_pixel_data`) | `___BUGHUNT___/016.../repro`, built with the `openjp2` Cargo feature (a pure-Rust JPEG 2000 backend - builds cleanly with just `cc`/`cmake` available, no external system library needed). Used a real JPEG 2000-encoded DICOM file (`dcms/examples_jpeg2k.dcm`, a genuine 480x640, 8-bit, 3-samples-per-pixel RGB image - embedded in this candidate's own `repro/inputs/` via `include_bytes!` for full self-containment, since the file lives in an untracked directory in the main checkout), then overwrote its `Rows`/`Columns` DICOM attributes to `4`/`4` via the public element-mutation API before decoding - the embedded JPEG 2000 codestream itself is untouched and still genuinely encodes a 480x640 image. | Reproduced deterministically: `range end index 49 out of range for slice of length 48` panicking in `decode_frame`, from a real, unmodified JPEG 2000 codestream simply paired with DICOM metadata that disagrees with it - no crafted/malformed codestream needed at all, just two independent, ordinarily-consistent pieces of data (container metadata vs. embedded format-native dimensions) made to disagree, which nothing in the code path ever checks for. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` | 3/3 runs reproduced identically - deterministic (a real, static compressed codestream; no randomness). |
| Regression check | Ran the same real `examples_jpeg2k.dcm` file *without* modifying its Rows/Columns through the full `decode_pixel_data()` -> `to_vec::<u8>()` pipeline. | Decoded successfully to 921,600 bytes (`480 * 640 * 3`, exactly as expected) both before and after the fix - confirms the fix doesn't reject genuinely consistent data. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed.

## Class

`crash` (an unguarded slice-range panic; reachable without needing any malformed/crafted JPEG 2000
codestream at all - only a mismatch between a DICOM file's own `Rows`/`Columns` metadata and the
dimensions genuinely baked into its embedded image data, two independent pieces of information
that the format allows to diverge and that nothing in this code path cross-checks before trusting
both).

## Root cause

`decode_frame` pre-allocates its output buffer from the DICOM object's declared `Rows` * `Columns`
* `SamplesPerPixel` * bytes-per-sample - metadata attributes that live in the DICOM data set,
entirely separate from the JPEG 2000 codestream's own embedded width/height (which the `jpeg2k`
crate decodes independently and has no knowledge of the surrounding DICOM object at all). The code
then iterates over the *decoded image's own* per-component sample count and writes each sample at
an offset derived from that count, with no check that it actually fits within the buffer sized
from the *DICOM metadata's* dimensions. Whenever these two independent dimension sources disagree
- whether from a malformed/malicious file, or simply a real-world producer bug that miscomputes
one of them - the write offsets exceed the buffer's real bounds and panic.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a check, right before the per-sample write loop, that each decoded component's actual sample
count (`component.data().len()`) does not exceed what the DICOM object's own declared `Rows` *
`Columns` calls for - returning a clear `Result::Err` (`ensure_whatever!`) identifying the
mismatch instead of writing past the buffer. Also corrected the adjacent
"too many components" bounds check from `component_i > samples_per_pixel` to `component_i >=
samples_per_pixel` (an off-by-one that would have let one more component through than the valid
`0..samples_per_pixel` index range allows - not independently confirmed as separately reachable,
but clearly incorrect on inspection and fixed alongside the main issue since it's the same
"validate against declared dimensions" area of the function). See `patch.diff`
(`transfer-syntax-registry/src/adapters/jpeg2k.rs`, one file).

Verified: `repro/` re-run against the patched tree no longer panics - `decode_pixel_data()` now
returns a clear error identifying exactly which component decoded to how many samples versus how
many were expected. A second, unmodified run against the same real file (Rows/Columns left as the
genuine 480/640) still decodes successfully to the correct 921,600 bytes, confirming the fix
doesn't affect legitimately consistent data.

- Project test suite: `cargo test -p dicom-transfer-syntax-registry -p dicom-pixeldata --lib
  --features dicom-transfer-syntax-registry/openjp2` and `cargo test --workspace --lib` - 351
  passed / 0 failed / 2 ignored (workspace-wide, `openjp2` not part of default features) and 39 +
  3 passed / 0 failed / 2 ignored (with `openjp2` explicitly enabled), identical both before and
  after the patch (no regressions, no newly-passing tests - no existing test constructs a
  dimension mismatch between DICOM metadata and an embedded JPEG 2000 codestream).

## What's not done / open questions

- Only `Jpeg2000Adapter::decode_frame` was checked and fixed - `Jpeg2000Adapter` has no plain
  `decode()` method of its own (only `decode_frame`, per the trait's default-provided `decode()`
  which likely calls `decode_frame` per frame - not independently re-verified that the default
  path doesn't have its own separate issue).
- Did not check `jpegxl.rs`/`jpegls.rs` for the identical class of bug (pre-sizing an output
  buffer from DICOM metadata, then writing the *external codec's own* decoded sample count into
  it without cross-checking) - `jpeg2k.rs` was chosen first since it was flagged as the most
  likely candidate in `IDEAS.md`, but the same investigation approach (compare a real sample file
  decoded normally vs. with deliberately mismatched Rows/Columns) could be repeated for the others.
- Not fuzzed - found by direct code reading (the buffer-presizing-from-metadata pattern, cross-
  referenced against `HEURISTICS.md`'s "arithmetic on untrusted sizes" and the general principle
  that a container format's own metadata and an embedded payload's native metadata are two
  independent trust boundaries), then confirmed using a real, unmodified JPEG 2000 file with only
  its DICOM-level metadata altered via the public API - not a fuzzer-style malformed-codestream
  attack, a much simpler "these two numbers don't have to agree" construction.
