# strict_decode_truncated_scan_hard_eof

Cluster candidate 011 (`strict_decode_vs_other_decoders_leniency_cluster`), seeded from
image-rs/jpeg-decoder issues #228, #229, #251, #257, #262: real-world JPEGs that other decoders
(libjpeg-turbo, ffmpeg, browsers, ImageMagick) decode fully or partially, but `jpeg-decoder`
rejects with a hard `Err`. Downloaded every image actually attached to those issues (including
#257's 27-image `broken_images.zip` bundle) and ran them through the crate's real public
`Decoder::decode()` API to see which errors were genuinely reproducible and how common each was.

This candidate narrows the cluster to its single largest, best-defined sub-bug: the
`Error::Io(UnexpectedEof)` / "failed to fill whole buffer" error, which affected 22 of the 33
downloaded real-world images (#251, #262, and 20 of #257's 27 attached images) - by far the most
common failure mode in the cluster. The other error classes present in the same sample set ("FF 00
found where marker was expected" - #228, "failed to decode huffman code" - #229, "RES marker found
where not allowed", "SOI marker found where not allowed", "no marker found where RSTn was
expected") are distinct root causes in different code paths and are left open as separate suspects
(see "What's not done" below and `IDEAS.md`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential run against real-world sample corpus | Release build (`cargo +stable build --release --example`), real public `Decoder::decode()` API, 33 real JPEGs downloaded directly from the 5 linked GitHub issues (individual attachments from #228/#229/#251/#262/#169, plus all 27 images in #257's `broken_images.zip`) | 22/33 files failed with `Error::Io` "failed to fill whole buffer"; the rest failed with 4 other distinct `Error::Format` messages (see above), and 1 succeeded outright (#169, already known-lenient per that issue) |
| Code reading (`src/huffman.rs::HuffmanDecoder::read_bits`, `src/decoder.rs::decode_internal`/`decode_scan`) | n/a | Found the exact mechanism (below); root-caused without needing miri/ASAN/fuzzing - this is a plain I/O-propagation logic issue, not memory-unsafety, so those tools would not have added signal (see `HEURISTICS.md` "Logic bugs" guidance) |
| Boundary/regression check after fix: full corpus re-run + project test suite before/after | Release build, same 33-file corpus + `cargo +stable test --release` | 21/22 "failed to fill whole buffer" files now decode successfully to a correctly-sized pixel buffer; 1 (`e881bf9e...jpg`) now fails with a clearer `Error::Format` ("no marker found where RST0 was expected") instead of the opaque I/O error - a truncation that happens to land exactly where a restart marker was expected, a known-open edge case (see below); none of the other 10 files (the 4 unrelated error classes) changed at all; project test suite unchanged (23/23 passing both before and after) |

Two escalating approaches were enough to reach a full root cause and a verified fix; a third
(fuzzing/miri/ASAN) was not needed - this is exactly `HEURISTICS.md`'s "logic bug, not
crash/UB" case, where differential testing against real inputs plus code reading is the right tool,
not a memory-safety instrument.

## Status

**Confirmed**, reproduces 2/2 runs (release build, deterministic - no seed/layout dependency,
input read from a real file via `BufReader`).

**Fully reproducible**: **Yes**. The repro feeds a real, unmodified, truncated JPEG - the actual
file attached to issue #251 by its reporter - into the crate's own public `Decoder::new(reader)` +
`decoder.decode()` entry point, the same path any real caller (an image viewer, the `image` crate)
uses to open an arbitrary file. Per `CONFIRMATION.md` section 11, this is the "single external
artifact, no second participant to fake" case: a malformed file *is* the genuine external input,
there is nothing else to fake.

## Class

logic (overly strict error propagation - the file is genuinely truncated, but far more of it was
already correctly decoded than the hard error communicates, and other real-world decoders return
that partial result instead of nothing)

## Root cause

`HuffmanDecoder::read_bits` (`src/huffman.rs`) refills its 64-bit bit buffer by calling
`read_u8(reader)?` directly, which propagates any `io::Error` - including a completely mundane
`ErrorKind::UnexpectedEof` ("failed to fill whole buffer") from `Read::read_exact` when the
underlying reader (a `File`/`BufReader`/byte slice) simply runs out of bytes - straight up through
`Result<()>` as a hard `Error::Io`, aborting the entire `decode()` call and discarding every pixel
already correctly decoded.

This matters because truncated/corrupted entropy-coded scan data is common in real-world JPEGs
(files clipped by a bad download, an aborted save, a lossy copy, or genuinely malformed encoders) -
and every other tested decoder (libjpeg-turbo, ffmpeg, browsers, ImageMagick, per the linked
issues) treats running out of scan data as an implicit end of the entropy-coded segment: decode as
many blocks/MCUs as the available bits actually support and return the (possibly visually
degraded, e.g. gray/black in the truncated region) result, rather than discarding the whole image.
`jpeg-decoder`'s own `read_bits` already had exactly this "treat missing data as zero bits" logic
for one case - encountering a real marker byte (`Some(_) => 0` in the original code) - it just
didn't extend the same treatment to hitting genuine end-of-input.

A second, necessary half of the same bug lives in `Decoder::decode_internal` (`src/decoder.rs`):
even if `read_bits` is patched to zero-fill on EOF, the scan-ending code
(`HuffmanDecoder::take_marker`) will then never find a real terminating marker either (there isn't
one - the file is truncated), so it returns `None`. The pre-existing main decode loop had no
handling for that case: it would take `pending_marker = None`, loop back to the top, call
`self.read_marker()` again on the same already-exhausted reader, and hit the identical
`UnexpectedEof` a second time from a different call site - so patching `read_bits` alone is not
sufficient on its own.

## Introduced by

1333a77 "initial commit" (2016-02-10). `read_bits`' unconditional `read_u8(reader)?` on the
non-marker branch, and `decode_internal`'s marker-driven scan loop with no handling for a scan that
ends without finding one, are both present in the very first commit of this crate - this has been
the behavior since the project's inception, not a later regression. (`git log -S"fn read_bits"` and
`git blame` on both affected code spans confirm this; verified against the full history of both
`src/huffman.rs` and `src/decoder.rs`.)

## The fix

Two coordinated changes (see `patch.diff`):

1. `src/huffman.rs`: `HuffmanDecoder` gains an `eof: bool` field. `read_bits`' primary byte read is
   wrapped so that an `io::Error` of kind `UnexpectedEof` sets `eof = true` and treats the byte as
   `0` (extending the existing "zero-fill once a marker has been found" behavior to "zero-fill once
   the reader is genuinely exhausted, too") instead of propagating the I/O error. Any other I/O
   error kind is still propagated unchanged - only exhaustion is treated as lenient.
2. `src/decoder.rs`: in `decode_internal`'s `Marker::SOS` handling, after a scan returns with
   `pending_marker = marker`, if `marker` is `None` (no terminating marker could be found - i.e.
   the truncation case above), the main marker loop now `break`s immediately instead of looping
   back into `read_marker()` on the exhausted reader. This mirrors the existing `Marker::EOI =>
   break` handling for a normal end of image, and lets the existing (pre-existing, unmodified)
   `decode_planes` finalization - which already renders whatever's available for an unfinished
   progressive component - produce the best-effort result for baseline images too.

Verified against more than the single triggering input (CONFIRMATION.md section 7): the fix was
checked against the full 33-image corpus (not just #251's file used in `repro/`), covering multiple
distinct truncation points/scan structures (baseline and progressive, interleaved and
non-interleaved, with and without restart intervals) - 21 of the 22 previously-hard-erroring images
now decode fully; the fix's own comment and root cause explicitly note the 1 remaining case
(truncation landing exactly where a restart marker was expected) is a related-but-distinct scenario
not covered by this change (see "What's not done" below).

- Project test suite: `cargo +stable test --release` - **23 passed, 0 failed, both before and
  after** the patch (identical counts; no regressions). This includes `tests/reftest::reftest`,
  which pixel-compares decoder output against reference PNGs for the existing well-formed test
  corpus, giving confidence the change doesn't alter output for any non-truncated file.

## What's not done / open questions

- One image in the corpus (`e881bf9e898c2383818f18d9972fb18b_full.jpg` from #257) is truncated
  exactly at a point where a restart (`RSTn`) marker was expected mid-scan. That specific code path
  (`decode_scan`'s `mcus_left_until_restart == 0` branch, `src/decoder.rs` around line 920) still
  returns a hard `Error::Format("no marker found where RSTn was expected")` rather than falling
  back to the same best-effort completion - this fix does not touch that branch. Worth a follow-up
  candidate specifically about restart-interval-boundary truncation.
- The other 4 error classes seen in the same corpus ("FF 00 found where marker was expected" -
  #228, "failed to decode huffman code" - #229, "RES marker found where not allowed", "SOI marker
  found where not allowed") are distinct root causes in different code (the FF-00/marker-detection
  logic inside the same `read_bits`, and the top-level marker dispatch in `decode_internal`) and
  were deliberately left untouched by this fix, per this skill's one-bug-per-candidate rule. Each
  is a plausible follow-up candidate; the raw per-file results are preserved in this session's
  working notes and summarized in `IDEAS.md`.
- This fix does not attempt to make the *lossless* (`decoder/lossless.rs`) scan path lenient to the
  same class of truncation - only the DCT-based `decode_scan`/baseline+progressive path was in
  scope here, since none of the sample corpus exercised lossless JPEGs.
- No attempt was made to surface a warning/diagnostic distinguishing "decoded cleanly" from
  "decoded via truncation leniency" (libjpeg does this via a return code /
  `jpeg_read_scanlines`-adjacent warning). `jpeg-decoder`'s public API has no such channel today;
  adding one would be a larger API-surface change out of scope for a bounded fix.
