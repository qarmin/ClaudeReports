# ts-registry: excessive allocation safeguard in codec adapters (jpeg/jpeg2k/jpegxl/rle_lossless)

From `IDEAS.md`'s backlog: an unmerged upstream branch (`upstream/bug/safeguard-excessive-allocs`,
commits `10f9b6db`..`4357bbe9`, none ancestors of current `master`/`df2b9e42`) adds a
`transfer-syntax-registry::alloc` safeguard module and a `DecodeError::Allocate` variant, and
routes `transfer-syntax-registry/src/adapters/{jpeg,jpeg2k,jpegxl,rle_lossless}.rs` through it.
This extends the already 3-for-3 confirmed untrusted-length-preallocation class (candidates
`004`/`080`/`081`, in `parser/`) into pixel-data codec adapters - exactly the gap noted in
`IDEAS.md`: "codec-specific fragment/scanline buffers in ... `pixeldata/src/` may have the same
`Vec::with_capacity`/`smallvec!`-from-untrusted-length pattern ... not checked specifically for
this allocation pattern".

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | All four codec adapter files in `transfer-syntax-registry/src/adapters/` (current `master`). Each pre-sizes its output buffer via `dst.resize(...)`/`dst.reserve_exact(...)` directly from `Rows`/`Columns`/`SamplesPerPixel`/`BitsAllocated`/`NumberOfFrames` - attributes read straight from the DICOM object's own metadata (`PixelDataObject` trait, `u16`/`u32` fields, fully attacker-controlled), with no check against the actual encapsulated fragment's size. `jpeg2k.rs:59/61`, `rle_lossless.rs:57/168`, `jpeg.rs:49/168`, `jpegxl.rs:68` all confirmed to use this pattern (`jpegxl.rs` line 106 already used a fallible `try_reserve` for one *later* buffer, but the *first* `dst.reserve_exact` at line 68 is still infallible). |
| Standalone repro (release build, `dicom-encoding` + `dicom-transfer-syntax-registry`'s public API: a minimal `PixelDataObject` implementation + `TransferSyntaxRegistry::get(..).pixel_data_reader().decode()` - the same real dispatch path `dicom-pixeldata`'s `decode_pixel_data()` uses internally) | `___BUGHUNT___/085.../repro`. A `PixelDataObject` declaring `Rows=Columns=SamplesPerPixel=65535`, `BitsAllocated=16` (implying a `~512 TiB` frame) paired with one genuinely tiny, well-formed 16-byte RLE fragment (`NumberOfSegments=1`, one segment spanning the whole fragment) - nothing malformed about the fragment itself, only a disagreement between declared header metadata and actual payload size. | Reproduced deterministically on both the main project's working tree and a pristine clone at `df2b9e42`: `memory allocation of 562924184010750 bytes failed`, process aborts via `SIGABRT` (exit code 134) - no `ulimit` trick needed, since 512 TiB fails to allocate on any ordinary machine outright. |
| Repeat-run check | `for i in 1..3: ./target/release/repro` (both on the main tree and via `just reproduce-before` against the pristine clone) | 3/3 runs aborted identically - deterministic, no randomness anywhere in this path. |
| Cross-check of sibling adapters | Grepped all four adapter files for `reserve_exact`/`resize`/`try_reserve` | Confirmed the exact same "pre-size from declared metadata, no bound against actual data" pattern in `jpeg.rs`, `jpeg2k.rs`, and `jpegxl.rs` as well, not just `rle_lossless.rs` (the one file with a standalone runnable repro built this session) - see "What's not done" below. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, no fixed seed needed, reproduced both on
the main project's working tree and from a genuinely pristine clone at the stated base commit.

**Fully reproducible**: **Yes**. The repro drives the real, public `PixelDataReader::decode()`
entry point (the exact dispatch `dicom-pixeldata`'s `decode_pixel_data()` uses via
`TransferSyntaxRegistry`) with a single external artifact (a `PixelDataObject` whose declared
attributes and one real fragment disagree in size) - there is no second participant being faked
here (per `CONFIRMATION.md` section 11's carve-out: a single malformed/inconsistent input handed
to a parser's real public entry point needs no second participant to impersonate).

## Class

`crash` / `security` (uncatchable process abort via Rust's default global-allocator-failure
handler - not a panic interceptable by `catch_unwind` or an outer `Result` chain - triggered by
nothing more than a DICOM object's own header attributes disagreeing with its actual pixel data
payload size; a natural DoS vector for any service decoding untrusted DICOM pixel data).

## Root cause

`RleLosslessAdapter::decode`/`decode_frame` (and, by the same pattern, `JpegAdapter::decode`/
`decode_frame`, `Jpeg2000Adapter::decode_frame`, and the first buffer in
`JpegXlAdapter::decode_frame`) compute `frame_size = bytes_per_sample * cols * rows *
samples_per_pixel` straight from the DICOM object's own `Rows`/`Columns`/`SamplesPerPixel`/
`BitsAllocated` attributes - entirely independent of how large the actual encapsulated fragment
data is - and then call `Vec::resize`/`reserve_exact` with that size using Rust's default
infallible allocation path. If the allocator can't satisfy the request (as it can't for the
~512 TiB constructed here, and would not be able to on essentially any real deployment for
declared dimensions anywhere near the format's own maximum `u16` range), Rust's global
`handle_alloc_error` aborts the whole process immediately - the same failure mode already
confirmed independently three times in `parser/src/stateful/decode.rs` (candidates `004`/`080`/
`081`), here in a different crate/layer (pixel data codec adapters) that nothing in the earlier
sessions had checked yet.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Applied the relevant hunks of the unmerged upstream branch (`10f9b6db`, `6d858892`, `361c3aaa`,
`30934ea4`, `761ca537`, `51a083a0`, `e14df40a`, `4357bbe9` - the full combined diff against
`encoding/src/adapters.rs`, `transfer-syntax-registry/src/{lib.rs,alloc.rs,adapters/{jpeg,jpeg2k,
jpegxl,rle_lossless}.rs}` applied cleanly with `git apply` against current `master`, no conflicts)
directly to the working tree:

- A new `pub(crate)` `transfer-syntax-registry::alloc` module (`guarded_alloc`/`guarded_reserve`/
  `guarded_resize`) that first rejects an allocation as "dangerous" if it's both above a 16 MiB
  floor *and* more than 32x larger than the actual encapsulated/fragment size actually available,
  then performs the allocation via `try_reserve_exact` (a normal, catchable `Result::Err` instead
  of an abort) for anything that passes that check.
- All four adapters' initial output-buffer sizing now goes through `guarded_resize`/
  `guarded_reserve`/`guarded_alloc` instead of raw `dst.resize`/`dst.reserve_exact`, surfacing a
  new `DecodeError::Allocate { name, size }` variant on failure instead of aborting.

Verified: re-running the standalone `repro/` against the patched tree (both the main working tree
and a pristine `df2b9e42` clone with `patch.diff` applied via `just apply`) no longer aborts - it
now prints a normal `Result::Err(Allocate { name: "frame", size: Some(562924184010750) })` and
exits 0. Re-ran 3/3 times, identical each time. A regression check against real, well-formed RLE
Lossless DICOM fixtures (`transfer-syntax-registry/tests/rle.rs`, added by the same upstream
branch, using real `dicom-test-files` samples `WG04/RLE/CT1_RLE` and a second RGB fixture) still
decodes correctly with the patch applied (`2 passed; 0 failed`), confirming the fix doesn't reject
legitimately consistent data - the 32x ratio threshold and 16 MiB floor are comfortably above any
genuine RLE/JPEG/JPEG2000/JPEG-XL compression ratio for real medical images.

- Project test suite: `cargo test --workspace --lib` - **351 passed / 0 failed / 2 ignored**,
  identical before and after the patch (summed across all 12 crates with lib tests; the workspace
  default features don't include `rle`/`jpeg`/`jpegxl`/`openjp2`, so this doesn't exercise the
  patched code directly - see the feature-gated checks below). With `--features rle`: `cargo test
  -p dicom-transfer-syntax-registry --features rle --test rle` - **2 passed / 0 failed** (this
  integration test only exists on the patched tree, since it was added by the same upstream
  branch - there is no "before" run for it). `cargo build -p dicom-transfer-syntax-registry
  --features rle,jpeg` / `--features jpegxl,jxl-oxide` / `--features openjp2` all build cleanly
  with the patch applied.

See `patch.diff` (6 files: `encoding/src/adapters.rs`, `transfer-syntax-registry/src/lib.rs`,
`transfer-syntax-registry/src/alloc.rs` (new), and the 4 adapter files).

## What's not done / open questions

- Only `rle_lossless.rs` got a standalone runnable `repro/` this session (it was the simplest to
  drive with a minimal `PixelDataObject` mock and a hand-built fragment, and needs no optional
  codec dependency feature). `jpeg.rs`, `jpeg2k.rs`, and `jpegxl.rs` were confirmed by direct code
  reading to share the exact same pre-sizing-from-declared-metadata pattern (see "What was tried"
  above, cross-check row) and are covered by the same applied `patch.diff`, but were not
  independently reproduced with their own minimal inputs this session - a future session could
  build one repro per adapter (would need the `jpeg-decoder`/`jpeg2k`(openjp2)/`jxl-oxide` feature
  crates as real dependencies, unlike `rle_lossless.rs` which is pure hand-rolled logic with no
  external codec crate needed to reach the vulnerable line).
- `jpegxl.rs` has a *second*, already-fallible `dst.try_reserve` for its 8-bit sample path (line
  106 on `master`) that predates this branch - only the *first* `dst.reserve_exact` (line 68,
  sized from declared dimensions before any codec decode happens at all) was the confirmed gap;
  the branch's `guarded_reserve` replaces both, but only the first was independently verified as
  unsafe on `master` before the fix.
- This is the same allocation-failure-aborts-the-process failure mode as candidates `004`/`080`/
  `081` (`parser/src/stateful/decode.rs`), just in a different crate/layer - IDEAS.md's backlog
  entry noted in "Confirmed bug classes" this closes the loop on is now resolved; no further
  codec-adapter files in this crate were left unchecked (all four flagged by the candidate
  description were read and patched).
