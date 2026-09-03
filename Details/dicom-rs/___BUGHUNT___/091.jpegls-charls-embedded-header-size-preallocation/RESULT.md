# jpegls: charls decoder's embedded-header size preallocation aborts the process

From `IDEAS.md`'s backlog: candidate 085 (`085.ts-registry-excessive-alloc-safeguard`) confirmed
the untrusted-declared-size-preallocation abort class in `jpeg.rs`/`jpeg2k.rs`/`jpegxl.rs`/
`rle_lossless.rs`, all of which pre-size their output buffer directly from the DICOM object's own
`Rows`/`Columns`/`SamplesPerPixel`/`BitsAllocated` attributes via an infallible `Vec::resize`/
`reserve_exact`. `jpegls.rs` (`transfer-syntax-registry/src/adapters/jpegls.rs`) was explicitly out
of scope for that candidate and had never been checked. Reading it shows it does **not** follow
that exact pattern - `JpegLsAdapter::decode_frame` never pre-sizes anything itself; it just calls
`charls::CharLS::default().decode(&frame_data)`. But `charls` (the external Rust bindings crate,
version 0.4.2, wrapping the C++ CharLS library) does exactly this internally, one layer down: its
`decode_with_stride` parses the JPEG-LS codestream's own `SOF55` (Start Of Frame) marker segment,
computes a destination size straight from that marker's `Height`/`Width`/`ComponentCount`/
`BitsPerSample` fields, and then does `let mut dst = vec![0; size];` - the identical infallible
allocation class as candidate 085, just driven by the *encapsulated fragment's own embedded
header* instead of a DICOM data-set attribute, and one dependency layer removed from `jpegls.rs`
itself.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `transfer-syntax-registry/src/adapters/jpegls.rs` (current `master`) and the `charls` crate's source (`~/.cargo/registry/.../charls-0.4.2/src/lib.rs`, `charls-sys-2.4.4/charls/src/charls_jpegls_decoder.cpp`) | Confirmed `jpegls.rs` itself has no `Vec::resize`/`reserve_exact` call at all in `decode_frame` - the suspected exact pattern from candidate 085 does not exist here. But `charls::Decoder::decode_with_stride` (`charls-0.4.2/src/lib.rs:105-158`) calls `charls_jpegls_decoder_read_header` (parses the codestream's own SOF55 marker), `charls_jpegls_decoder_get_destination_size` (computes `component_count * height * width * bytes_per_sample`, `charls_jpegls_decoder.cpp:82-97`, from those same header fields), then `let mut dst = vec![0; size]` (`charls-0.4.2/src/lib.rs:139`) - an infallible allocation sized entirely from the codestream's own embedded header, before any entropy-coded scan data is validated. |
| Direct FFI probe (`charls`/`charls-sys` crates, standalone scratch project, release build) | Used `charls`'s own encoder to produce a genuine, valid JPEG-LS codestream (4x4, N components, 16 bits/sample), then patched only the `Height`/`Width` fields of its `SOF55` marker segment from `4`/`4` to `0xFFFF`/`0xFFFF` (all other bytes, including every scan header, are exactly what `charls`'s own encoder produced) | Calling raw `charls_jpegls_decoder_get_destination_size` after `read_header` on the patched stream returns `err=0` with a huge `size` (e.g. 34,358,689,800 bytes for 4 components) - confirming `read_header` accepts the patched dimensions and the size computation runs to completion using them, unmodified by anything downstream. Sweeping `component_count` from 1 to 8: sizes from 8.6 GB to 68.7 GB; from `component_count=5` (42.9 GB) upward, calling the crate's real `.decode()` on this machine (24 GiB RAM+swap available) aborts with `memory allocation of N bytes failed` / SIGABRT; below that (`component_count<=4`, <=34.3 GB) it doesn't abort here (the OS/allocator tolerates the virtual reservation on this machine, then `decode_to_buffer` fails gracefully on the deliberately-garbage entropy data) - the crash threshold is dependent on actual available memory, as expected for an allocation-failure abort, not for a fixed/deterministic input size. |
| Standalone repro (release build, `dicom-encoding` + `dicom-transfer-syntax-registry`'s public API via `TransferSyntaxRegistry::get("1.2.840.10008.1.2.4.80").pixel_data_reader().decode_frame()` - the exact real dispatch `dicom-pixeldata`'s `decode_pixel_data()` uses, `charls` feature enabled) | `___BUGHUNT___/091.../repro`. A `PixelDataObject` declaring ordinary, small, self-consistent `Rows=Columns=4`, `SamplesPerPixel=1`, `BitsAllocated=16` (nothing suspicious about the DICOM attributes themselves), paired with one real 989-byte fragment: a genuine `charls`-encoder-produced JPEG-LS codestream (`component_count=64`) whose `SOF55` `Height`/`Width` fields alone were patched to `0xFFFF`/`0xFFFF` -> declared frame size `65535*65535*64*2 = 549,739,036,800` bytes (~512 GiB), an amount no ordinary machine can satisfy | Reproduced deterministically on the main project's working tree (charls feature): `memory allocation of 549739036800 bytes failed`, SIGABRT, exit code 134 - the whole process aborts uncatchably from a 989-byte input handed to the real, public `PixelDataReader::decode_frame` entry point. |
| Repeat-run check | `for i in 1..3: ./target/release/repro`, both before the fix (pristine tree) and after (patched tree, via `git stash`/`git stash pop` around the main project's own `jpegls.rs`) | Before: 3/3 runs aborted identically (SIGABRT, exit 134). After: 3/3 runs completed normally with a graceful `Result::Err`, exit 0. |

## Status

**Confirmed** - deterministic, 3/3 runs, release build, `charls` feature, no fixed seed needed,
reproduced on the main project's working tree via the real public decode entry point.

**Fully reproducible**: **Yes**. The repro drives the real, public `PixelDataReader::decode_frame()`
entry point (the same dispatch `dicom-pixeldata`'s `decode_pixel_data()` uses via
`TransferSyntaxRegistry`) with a single external artifact - one DICOM object whose own attributes
are small and self-consistent, carrying one encapsulated pixel fragment whose *own embedded
JPEG-LS frame header* disagrees with its actual byte length. Per `CONFIRMATION.md` section 11's
carve-out, this needs no second participant to impersonate: a malformed/self-inconsistent file (or
here, a malformed/self-inconsistent encapsulated fragment within an otherwise ordinary file) handed
to a parser's real public entry point *is* the genuine external input a real attacker/user would
supply.

## Class

`crash` / `security` (uncatchable process abort via Rust's default global-allocator-failure
handler, not a panic interceptable by `catch_unwind` or an outer `Result` chain - triggered by a
989-byte encapsulated JPEG-LS fragment with no invalid DICOM header attributes at all, only an
internal disagreement between its own embedded frame header and its own actual length; a natural
DoS vector for any service decoding untrusted DICOM pixel data with the `charls` feature enabled).

## Root cause

`JpegLsAdapter::decode_frame` (`transfer-syntax-registry/src/adapters/jpegls.rs`) itself does not
preallocate anything from untrusted DICOM attributes - it defers entirely to
`charls::CharLS::default().decode(&frame_data)`. That function, inside the external `charls` crate
(`charls-0.4.2`, wrapping the C++ CharLS library via `charls-sys`), parses the JPEG-LS codestream's
own `SOF55` marker segment (`Height`, `Width`, `ComponentCount`, `BitsPerSample` - 1+2+2+1 = 6
bytes right after the marker's 2-byte segment length), computes
`destination_size = height * width * component_count * bytes_per_sample` from those fields alone
(`charls_jpegls_decoder_get_destination_size`, no relationship whatsoever to the DICOM object's own
`Rows`/`Columns`/`SamplesPerPixel`/`BitsAllocated`, and no check against how much actual codestream
data is present), and then calls `vec![0; destination_size]` using Rust's default infallible
allocation path - before a single entropy-coded sample has been decoded or validated. If the
allocator can't satisfy the request (trivially true for the 512 GiB constructed here, and true for
essentially any real deployment given declared dimensions anywhere near the format's own `u16`
maximum), Rust's global `handle_alloc_error` aborts the whole process immediately. This is the same
process-abort-on-untrusted-size failure mode independently confirmed in `parser/src/stateful/
decode.rs` (candidates `004`/`080`/`081`) and in the sibling codec adapters (candidate `085`) - here
one dependency layer removed (inside `charls` itself, not `dicom-rs`'s own adapter code), and driven
by the pixel fragment's *own* embedded codec header rather than by any DICOM data-set attribute.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Added a bounded, self-contained guard directly in `jpegls.rs` (no change to the `charls` crate or
its `Cargo.toml` dependency needed): a small pure-Rust JPEG marker-segment scanner
(`jpegls_declared_frame_size`) that finds the codestream's own `SOF55` marker and computes the same
`height * width * component_count * bytes_per_sample` value `charls`'s decoder is about to use,
*before* calling into `charls` at all. `check_declared_frame_size` then rejects the fragment with a
normal, catchable `DecodeResult::Err` (via the existing `ensure_whatever!` macro already used
elsewhere in this file, no new error variant needed) whenever that declared size is both above a
16 MiB floor *and* more than 32x the size of the actual fragment bytes available - the same
floor/ratio shape as the `guarded_alloc` safeguard from candidate 085's (unmerged upstream) fix for
the sibling adapters, chosen for the same reason: no genuine JPEG-LS-compressible medical image
gets anywhere near a 32x compression ratio, so this cannot reject real data, only data whose header
and actual payload size are already wildly inconsistent. `decode_frame` now calls
`check_declared_frame_size(&frame_data)?` right after fetching the fragment, before calling into
`charls`. If the scanner can't parse a `SOF55` marker at all (truncated/garbled input), it returns
`None` and the guard is skipped - `charls`'s own header validation still runs and rejects malformed
markers on its own terms, unchanged from before this patch.

Verified:

- Re-running the standalone `repro/` against the patched tree (`git stash`/`git stash pop` around
  the main project's own `jpegls.rs`, 3 repeat runs each side): before the patch, 3/3 runs abort
  (`memory allocation of 549739036800 bytes failed`, SIGABRT, exit 134); after the patch, 3/3 runs
  complete normally with `Result::Err("... implausibly large frame (549739036800 bytes) versus the
  encapsulated fragment size (989 bytes) ...")`, exit 0. See `output.txt` for the real captured
  output of both.
- Varied the triggering input within the same bug class (per `CONFIRMATION.md` section 7, not just
  the exact fragment the original repro used): a second, independently-crafted fragment with
  `component_count=32` instead of `64` (declared size 274,869,518,400 bytes vs. 509 actual bytes)
  is rejected identically by the patched adapter with the same graceful error, confirming the guard
  isn't narrowly matched to one specific declared size.
- Real, well-formed JPEG-LS fixtures still decode correctly with the patch applied:
  `cargo test -p dicom-transfer-syntax-registry --features charls --test jpegls` -
  **3 passed / 0 failed**, identical before and after the patch (`read_jpeg_ls_1` and
  `read_jpeg_ls_lossless_1` against real `dicom-test-files` samples `WG04/JLSN/NM1_JLSN` and
  `pydicom/MR_small_jpeg_ls_lossless.dcm`, plus a round-trip encode/decode test) - the 32x
  ratio/16 MiB floor is comfortably above any genuine JPEG-LS compression ratio for real medical
  images, so legitimate data is never rejected.
- Project test suite: `cargo test --workspace --lib` - **351 passed / 0 failed / 2 ignored**,
  identical before and after the patch (the workspace's default features don't include `charls`,
  so this doesn't exercise the patched code directly, but confirms no unrelated regression).

See `patch.diff` (1 file: `transfer-syntax-registry/src/adapters/jpegls.rs`).

## What's not done / open questions

- The fix guards `decode_frame`'s call into `charls::decode`, the path this candidate actually
  reproduced. `JpegLsAdapter`/`JpegLsLosslessWriter`'s `encode_frame`/`encode` paths call
  `charls::CharLS::encode`, which allocates its output buffer from `charls_jpegls_encoder_
  get_estimated_destination_size` - driven by the frame info *the DICOM object itself declares*
  (`cols`/`rows`/`bits_stored`/`samples_per_pixel`, already validated/owned by the caller
  constructing the object to encode), not by anything an external attacker controls independently
  of that object, so it wasn't treated as the same untrusted-input class and wasn't probed this
  session.
- This closes the `jpegls.rs` gap IDEAS.md's backlog entry named specifically. The backlog entry's
  secondary/lower-priority part - generic frame/scanline buffers in `pixeldata/src/` possibly
  sharing the `Vec::with_capacity`/`smallvec!`-from-untrusted-length pattern - was not investigated
  this session; re-added to `IDEAS.md` as its own follow-up (see that file).
- The root cause technically lives inside the external `charls` crate's own `decode_with_stride`
  (an infallible `vec![0; size]` from header-declared size, matching the exact class this whole
  backlog entry has been tracking across `parser/` and `transfer-syntax-registry/`) - the fix
  applied here works around it entirely within `dicom-rs`'s own code (no upstream `charls` change
  needed), by rejecting the disproportionate input before it ever reaches `charls::decode`. A
  maintainer wanting to fix this at the source instead would need to file this upstream against
  the `charls` crate (or the C++ CharLS library it wraps) directly - out of scope for this
  candidate.
